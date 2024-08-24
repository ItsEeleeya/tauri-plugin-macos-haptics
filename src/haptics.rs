use objc::{class, msg_send, runtime::Object, sel, sel_impl};

/// Represents the different types of haptic feedback patterns.
///
/// See [Apple's documentation](https://developer.apple.com/documentation/appkit/nshapticfeedbackpattern)
#[repr(u64)]
pub enum NSHapticFeedbackPattern {
    /// A general haptic feedback pattern for alignment.
    Alignment = 0,
    /// A haptic feedback pattern for level changes.
    LevelChange = 1,
    /// A generic haptic feedback pattern.
    Generic = 2,
}

/// Specifies when the haptic feedback should be performed.
///
/// See [Apple's documentation](https://developer.apple.com/documentation/appkit/nshapticfeedbackmanager/performancetime)
#[repr(u64)]
pub enum NSHapticFeedbackPerformanceTime {
    /// The system chooses the most appropriate time for feedback
    Default = 0,
    /// Provide immediate haptic feedback.
    Now = 1,
    /// Provide haptic feedback after the next screen update.
    DrawCompleted = 2,
}

/// Wrapper for the NSHapticFeedbackManager Objective-C class.
///
/// See [Apple's documentation](https://developer.apple.com/documentation/appkit/nshapticfeedbackmanager)
pub struct NSHapticFeedbackManager {
    inner: *mut Object,
}

/// This module provides a basic Rust interface to the macOS haptic feedback system.
///
/// # Example
/// IMPORTANT!: Use this only in response to user-initiated actions.
///             Ideally, visual feedback, such as a highlight or appearance of an alignment guide, should accompany the feedback.
///
///
/// ```rust
/// fn provide_feedback() {
///     #[cfg(target_os = "macos")]
///     {
///         use tauri_plugin_macos_haptics::haptics::*;
///         // Performs a generic haptic feedback immediately.
///         let _ = NSHapticFeedbackManager::default_performer().perform(NSHapticFeedbackPattern::Generic, None);
///     }
/// }
/// ```
///
/// Note: Haptic feedback is only supported on macOS 10.11 (OS X El Capitan) and above.
impl NSHapticFeedbackManager {
    /// Returns the default performer for haptic feedback.
    ///
    /// This corresponds to the [defaultPerformer](https://developer.apple.com/documentation/appkit/nshapticfeedbackmanager/1441752-defaultperformer) class method.
    pub fn default_performer() -> Self {
        unsafe {
            let cls = class!(NSHapticFeedbackManager);
            let inner: *mut Object = msg_send![cls, defaultPerformer];
            Self { inner }
        }
    }

    /// Performs haptic feedback with the specified pattern and timing.
    ///
    /// This corresponds to the [performFeedbackPattern:performanceTime:](https://developer.apple.com/documentation/appkit/nshapticfeedbackperformer/1441738-perform) method.
    ///
    /// # IMPORTANT!
    /// Call this method only in response to user-initiated actions. Ideally, visual feedback, such as a highlight or appearance of an alignment guide, should accompany the feedback.
    ///
    /// # Arguments
    /// * `pattern` - The haptic feedback pattern to use.
    /// * `performance_time` - When to perform the haptic feedback. If None, defaults to `Now`.
    ///
    /// # Returns
    /// A Result indicating success or an error if the operation failed.
    pub fn perform(
        &self,
        pattern: NSHapticFeedbackPattern,
        performance_time: Option<NSHapticFeedbackPerformanceTime>,
    ) -> Result<(), tauri::Error> {
        let ptime = performance_time.unwrap_or(NSHapticFeedbackPerformanceTime::Now);
        unsafe {
            let _: () = msg_send![self.inner, performFeedbackPattern:pattern performanceTime:ptime];
        }
        Ok(())
    }
}
