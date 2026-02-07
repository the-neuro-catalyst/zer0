use chrono::Local;

use std::sync::{Arc, Mutex};

use tracing::{Event, Subscriber};

use tracing_subscriber::Layer;

use tracing_subscriber::layer::Context;

/// Shared buffer for TUI logs
pub struct LogRegistry {
    pub logs: Arc<Mutex<Vec<String>>>,
}

impl LogRegistry {
    pub fn new() -> Self {
        Self { logs: Arc::new(Mutex::new(Vec::new())) }
    }
}

impl Default for LogRegistry {
    fn default() -> Self {
        Self::new()
    }
}

pub struct TuiLoggerLayer {
    logs: Arc<Mutex<Vec<String>>>,
}

impl TuiLoggerLayer {
    pub fn new(logs: Arc<Mutex<Vec<String>>>) -> Self {
        Self { logs }
    }
}

impl<S> Layer<S> for TuiLoggerLayer
where
    S: Subscriber,
{
    fn on_event(&self, event: &Event<'_>, _ctx: Context<'_, S>) {
        let mut logs = self.logs.lock().unwrap();

        // Format the message
        let timestamp = Local::now().format("%H:%M:%S");
        let level = *event.metadata().level();
        let target = event.metadata().target();

        // Simple visitor to extract the message field
        let mut visitor = MessageVisitor::new();
        event.record(&mut visitor);

        let log_line = format!("[{}] {:<5} [{}] {}", timestamp, level, target, visitor.message);

        logs.push(log_line);

        // Keep buffer size under control
        if logs.len() > 100 {
            logs.remove(0);
        }
    }
}

struct MessageVisitor {
    message: String,
}

impl MessageVisitor {
    fn new() -> Self {
        Self { message: String::new() }
    }
}

impl tracing::field::Visit for MessageVisitor {
    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        if field.name() == "message" {
            self.message = format!("{:?}", value);
        }
    }

    fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
        if field.name() == "message" {
            self.message = value.to_string();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing_subscriber::prelude::*;

    #[test]
    fn test_tui_logger_layer() {
        let registry = LogRegistry::new();
        let layer = TuiLoggerLayer::new(registry.logs.clone());

        let subscriber = tracing_subscriber::registry().with(layer);

        tracing::subscriber::with_default(subscriber, || {
            tracing::info!("test log message");
        });

        let logs = registry.logs.lock().unwrap();
        assert_eq!(logs.len(), 1);
        assert!(logs[0].contains("INFO"));
        assert!(logs[0].contains("test log message"));
    }

    #[test]
    fn test_log_filtering() {
        let registry = LogRegistry::new();
        let layer = TuiLoggerLayer::new(registry.logs.clone());

        let filter = tracing_subscriber::EnvFilter::new("warn");
        let subscriber = tracing_subscriber::registry().with(filter).with(layer);

        tracing::subscriber::with_default(subscriber, || {
            tracing::info!("should not appear");
            tracing::warn!("should appear");
        });

        let logs = registry.logs.lock().unwrap();
        assert_eq!(logs.len(), 1);
        assert!(logs[0].contains("WARN"));
        assert!(logs[0].contains("should appear"));
    }
}
