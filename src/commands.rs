use tauri::command;

#[cfg(target_os = "macos")]
use crate::haptics::*;

impl From<u64> for NSHapticFeedbackPattern {
    fn from(value: u64) -> Self {
        match value {
            0 => NSHapticFeedbackPattern::Alignment,
            1 => NSHapticFeedbackPattern::LevelChange,
            2 => NSHapticFeedbackPattern::Generic,
            _ => NSHapticFeedbackPattern::Generic, // Default to Generic for unknown values
        }
    }
}

impl From<u64> for NSHapticFeedbackPerformanceTime {
    fn from(value: u64) -> Self {
        match value {
            0 => NSHapticFeedbackPerformanceTime::Default,
            1 => NSHapticFeedbackPerformanceTime::Now,
            2 => NSHapticFeedbackPerformanceTime::DrawCompleted,
            _ => NSHapticFeedbackPerformanceTime::Default, // Default for unknown values
        }
    }
}

#[command]
pub async fn is_supported() -> bool {
    crate::is_supported()
}

#[command]
pub async fn perform(pattern: u64, performance_time: u64) -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        NSHapticFeedbackManager::default_performer()
            .perform(pattern.into(), Some(performance_time.into()))
            .map_err(|e| e.to_string())
    }

    #[cfg(not(target_os = "macos"))]
    Err("This command is only supported on macOS.")
}
