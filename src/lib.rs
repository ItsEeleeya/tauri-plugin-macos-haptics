#[cfg(feature = "alignment-feedback-filter")]
use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, Mutex},
};

#[cfg(feature = "alignment-feedback-filter")]
use binding_handle::AlignmentFeedbackId;
#[cfg(feature = "alignment-feedback-filter")]
use objc2::rc::Retained;
#[cfg(feature = "alignment-feedback-filter")]
use objc2_app_kit::{NSAlignmentFeedbackFilter, NSAlignmentFeedbackToken};
use tauri::{
    plugin::{Builder, TauriPlugin},
    Resource, Runtime,
};

mod commands;

#[cfg(target_os = "macos")]
mod haptics;

#[cfg(target_os = "macos")]
mod binding_handle;

#[cfg(all(target_os = "macos", feature = "alignment-feedback-filter"))]
#[derive(Clone)]
pub(crate) struct AlignmentFeedbackFilterHandle {
    id: AlignmentFeedbackId,
    filter: Rc<RefCell<Retained<NSAlignmentFeedbackFilter>>>,
    tokens: Rc<RefCell<Retained<Vec<Box<dyn NSAlignmentFeedbackToken>>>>>,
}

/// # UNSAFE
///
/// We make sure it always runs on the main thread.
#[cfg(all(target_os = "macos", feature = "alignment-feedback-filter"))]
unsafe impl Sync for AlignmentFeedbackFilterHandle {}
#[cfg(all(target_os = "macos", feature = "alignment-feedback-filter"))]
unsafe impl Send for AlignmentFeedbackFilterHandle {}

#[cfg(feature = "alignment-feedback-filter")]
impl Resource for AlignmentFeedbackFilterHandle {
    fn close(self: Arc<Self>) {
        drop(self)
    }
}

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("macos-haptics")
        .invoke_handler(tauri::generate_handler![
            commands::is_supported,
            commands::perform,
            #[cfg(feature = "alignment-feedback-filter")]
            commands::create_alignment_feedback_filter,
            // commands::remove_alignment_feedback_filter,
        ])
        .build()
}

pub fn is_supported() -> bool {
    #[cfg(target_os = "macos")]
    unsafe {
        // NSHapticFeedbackManager was introduced with OS X El Capitan version 10.11
        // https://developer.apple.com/documentation/appkit/nshapticfeedbackmanager?language=objc
        objc2_app_kit::NSAppKitVersionNumber >= objc2_app_kit::NSAppKitVersionNumber10_11
    }

    #[cfg(not(target_os = "macos"))]
    false
}
