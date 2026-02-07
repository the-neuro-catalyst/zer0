use log::LevelFilter;

use tauri::{Builder, Runtime};

use tauri_plugin_log::{RotationStrategy, Target, TargetKind};
pub fn init<R: Runtime>(builder: Builder<R>) -> Builder<R> {
    builder.plugin(
        tauri_plugin_log::Builder::new()
            .targets([
                Target::new(TargetKind::Stdout),
                Target::new(TargetKind::Webview),
                Target::new(TargetKind::LogDir { file_name: Some("inspector.log".to_string()) }),
            ])
            .level(LevelFilter::Info)
            .max_file_size(50_000 /* bytes */) // Set the maximum size
            .rotation_strategy(RotationStrategy::KeepAll) // Enable log rotation
            .filter(|metadata| {
                metadata.target().starts_with("ui_lib")
                    || metadata.level() <= log::LevelFilter::Info
            })
            .format(|out, message, record| {
                out.finish(format_args!(
                    "[{}] [{}] [{}] {}",
                    chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                    record.level(),
                    record.target(),
                    message
                ))
            })
            .build(),
    )
}
