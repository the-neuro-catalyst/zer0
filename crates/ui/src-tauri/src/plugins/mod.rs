use tauri::{Builder, Runtime};

mod logger;
mod process;
#[cfg(not(any(target_os = "android", target_os = "ios")))]
mod single_instance;
mod store;
mod system;
pub mod tray;

pub fn register_plugins<R: Runtime>(mut builder: Builder<R>) -> Builder<R> {
    // Standard centralized plugin registry
    builder = logger::init(builder);
    builder = system::init(builder);
    builder = process::init(builder);

    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    {
        builder = single_instance::init(builder);
    }

    builder = store::init(builder);
    builder
}
