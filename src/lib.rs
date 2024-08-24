use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime,
};

mod commands;

#[cfg(target_os = "macos")]
extern crate objc;

#[cfg(target_os = "macos")]
pub mod haptics;

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("macos-haptics")
        .invoke_handler(tauri::generate_handler![
            commands::is_supported,
            commands::perform
        ])
        .build()
}

pub fn is_supported() -> bool {
    #[cfg(target_os = "macos")]
    unsafe {
        // NSHapticFeedbackManager was introduced with OS X El Capitan version 10.11
        // https://developer.apple.com/documentation/appkit/nshapticfeedbackmanager?language=objc
        cocoa::appkit::NSAppKitVersionNumber >= cocoa::appkit::NSAppKitVersionNumber10_11
    }

    #[cfg(not(target_os = "macos"))]
    false
}
