use tauri::{Builder, Runtime};

pub fn init<R: Runtime>(builder: Builder<R>) -> Builder<R> {
    builder.plugin(tauri_plugin_opener::init()).plugin(tauri_plugin_dialog::init())
}
