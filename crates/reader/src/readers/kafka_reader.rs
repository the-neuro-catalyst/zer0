use crate::error::DataReaderError;

use futures_util::StreamExt;

use rdkafka::config::ClientConfig;

use rdkafka::consumer::{Consumer, StreamConsumer};

use rdkafka::message::Message;

use schema::SchemaValue;

use std::pin::Pin;

use tracing::info;

#[allow(dead_code)]
pub async fn read_kafka_stream(
    brokers: &str,
    group_id: &str,
    topic: &str,
) -> Result<
    Pin<Box<dyn tokio_stream::Stream<Item = Result<SchemaValue<'static>, DataReaderError>> + Send>>,
    DataReaderError,
> {
    let consumer: StreamConsumer = ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .set("group.id", group_id)
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "true")
        .create()
        .map_err(|e| {
            DataReaderError::InternalError(format!("Kafka consumer creation failed: {}", e))
        })?;

    consumer
        .subscribe(&[topic])
        .map_err(|e| DataReaderError::InternalError(format!("Kafka subscription failed: {}", e)))?;

    info!("Subscribed to Kafka topic: {} at {}", topic, brokers);

    let consumer = std::sync::Arc::new(consumer);
    let stream_consumer = consumer.clone();

    // We use a stream that owns the consumer
    let stream = async_stream::stream! {
        let mut message_stream = stream_consumer.stream();
        while let Some(message_res) = message_stream.next().await {
            match message_res {
                Ok(m) => {
                    match m.payload_view::<[u8]>() {
                        None => yield Ok(SchemaValue::Null),
                        Some(Ok(s)) => {
                            // Try to parse as JSON first
                            if let Ok(json_val) = serde_json::from_slice::<serde_json::Value>(s) {
                                yield Ok(convert_json_to_schema(json_val));
                                continue;
                            }

                            // Fallback to string
                            let text = String::from_utf8_lossy(s).to_string();
                            yield Ok(SchemaValue::String(std::borrow::Cow::Owned(text)))
                        },
                        Some(Err(e)) => yield Err(DataReaderError::InternalError(format!("Error viewing Kafka payload: {:?}", e))),
                    }
                },
                Err(e) => yield Err(DataReaderError::InternalError(format!("Kafka error: {}", e))),
            }
        }
    };

    Ok(Box::pin(stream))
}

#[allow(dead_code)]
fn convert_json_to_schema(val: serde_json::Value) -> SchemaValue<'static> {
    match val {
        serde_json::Value::Null => SchemaValue::Null,
        serde_json::Value::Bool(b) => SchemaValue::Boolean(b),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                SchemaValue::Integer(i)
            } else if let Some(f) = n.as_f64() {
                SchemaValue::Float(f)
            } else {
                SchemaValue::Unknown
            }
        }
        serde_json::Value::String(s) => SchemaValue::String(std::borrow::Cow::Owned(s)),
        serde_json::Value::Array(a) => {
            SchemaValue::Array(a.into_iter().map(convert_json_to_schema).collect())
        }
        serde_json::Value::Object(o) => {
            let mut map = std::collections::HashMap::new();
            for (k, v) in o {
                map.insert(std::borrow::Cow::Owned(k), convert_json_to_schema(v));
            }
            SchemaValue::Object(map)
        }
    }
}
