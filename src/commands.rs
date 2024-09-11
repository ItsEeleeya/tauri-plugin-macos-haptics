use objc2_app_kit::{
    NSAlignmentFeedbackFilter, NSHapticFeedbackManager, NSHapticFeedbackPattern,
    NSHapticFeedbackPerformanceTime, NSHapticFeedbackPerformer,
};
use std::sync::Arc;
use tauri::command;
use tauri::ResourceId;
use tauri::State;

#[cfg(target_os = "macos")]
use crate::haptics::*;

#[command]
pub async fn is_supported() -> bool {
    crate::is_supported()
}

#[command]
pub async fn perform(pattern: u64, performance_time: u64) -> Result<(), String> {
    #[cfg(target_os = "macos")]
    unsafe {
        if let (Some(pattern), Some(performance_time)) = (
            NSHapticFeedbackPattern::from_raw(pattern),
            NSHapticFeedbackPerformanceTime::from_raw(performance_time),
        ) {
            NSHapticFeedbackManager::defaultPerformer()
                .performFeedbackPattern_performanceTime(pattern, performance_time);
            Ok(())
        } else {
            Err("Invalid pattern or performance time value".into())
        }
    }

    #[cfg(not(target_os = "macos"))]
    Err("This command is only supported on macOS.".into())
}

#[command]
pub fn create_alignment_feedback_filter(state: State<'_, ResourceId>) -> Result<u32, String> {
    #[cfg(target_os = "macos")]
    unsafe {
        // let filter = NSAlignmentFeedbackFilter::new();
        // let rid = state.write().unwrap().add(filter);
        // Ok(rid)
        Ok((0_0))
    }

    #[cfg(not(target_os = "macos"))]
    Err("This command is only supported on macOS.".into())
}

#[command]
pub fn perform_alignment_feedback(
    rid: u32,
    view: u64,
    previous_x: f64,
    aligned_x: f64,
    default_x: f64,
) -> Result<(), String> {
    #[cfg(target_os = "macos")]
    unsafe {
        // let state = state.read().unwrap();
        // if let Some(filter) = state.get::<NSAlignmentFeedbackFilter>(rid) {
        //     filter.alignmentFeedbackTokenForHorizontalMovementInView_previousX_alignedX_defaultX(
        //         view, previous_x, aligned_x, default_x,
        //     );
        //     Ok(())
        // } else {
        //     Err("Invalid filter resource ID".into())
        // }
        Ok(())
    }

    #[cfg(not(target_os = "macos"))]
    Err("This command is only supported on macOS.".into())
}
