use tauri::{Builder, Runtime};
pub fn init<R: Runtime>(builder: Builder<R>) -> Builder<R> {
    builder.plugin(tauri_plugin_process::init())
}
