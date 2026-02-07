use crate::error::DataReaderError;

use futures_util::StreamExt;

use lapin::{Connection, ConnectionProperties, options::*, types::FieldTable};

use schema::SchemaValue;

use std::pin::Pin;

use tracing::info;

#[allow(dead_code)]
pub async fn read_rabbitmq_stream(
    url: &str,
    queue: &str,
) -> Result<
    Pin<Box<dyn tokio_stream::Stream<Item = Result<SchemaValue<'static>, DataReaderError>> + Send>>,
    DataReaderError,
> {
    let conn = Connection::connect(url, ConnectionProperties::default()).await.map_err(|e| {
        DataReaderError::InternalError(format!("RabbitMQ connection failed: {}", e))
    })?;

    let channel = conn.create_channel().await.map_err(|e| {
        DataReaderError::InternalError(format!("RabbitMQ channel creation failed: {}", e))
    })?;

    let _queue = channel
        .queue_declare(queue, QueueDeclareOptions::default(), FieldTable::default())
        .await
        .map_err(|e| {
            DataReaderError::InternalError(format!("RabbitMQ queue declare failed: {}", e))
        })?;

    info!("Subscribed to RabbitMQ queue: {} at {}", queue, url);

    let mut consumer = channel
        .basic_consume(queue, "nc-reader", BasicConsumeOptions::default(), FieldTable::default())
        .await
        .map_err(|e| {
            DataReaderError::InternalError(format!("RabbitMQ basic consume failed: {}", e))
        })?;

    // Keep conn and channel alive as long as the stream exists
    let _conn = conn;
    let _channel = channel;

    let stream = async_stream::stream! {
        while let Some(delivery_res) = consumer.next().await {
            match delivery_res {
                Ok(delivery) => {
                    let payload = &delivery.data;

                    // Try to parse as JSON first
                    if let Ok(json_val) = serde_json::from_slice::<serde_json::Value>(payload) {
                        yield Ok(convert_json_to_schema(json_val));
                        continue;
                    }

                    // Fallback to string
                    let text = String::from_utf8_lossy(payload).to_string();
                    yield Ok(SchemaValue::String(std::borrow::Cow::Owned(text)))
                },
                Err(e) => yield Err(DataReaderError::InternalError(format!("RabbitMQ delivery error: {}", e))),
            }
        }
        // Ensure they are moved into the generator
        drop(_conn);
        drop(_channel);
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
