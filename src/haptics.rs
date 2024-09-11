use objc2::{ffi::nil, rc::Retained, runtime::ProtocolObject};
use objc2_app_kit::{
    NSAlignmentFeedbackFilter, NSAlignmentFeedbackToken, NSHapticFeedbackPattern,
    NSHapticFeedbackPerformanceTime,
};
use objc2_foundation::{NSArray, NSPoint};

pub trait FromRaw<T> {
    fn from_raw(value: T) -> Option<Self>
    where
        Self: Sized;
}

impl FromRaw<u64> for NSHapticFeedbackPattern {
    fn from_raw(value: u64) -> Option<Self> {
        match value {
            0 => Some(Self::Alignment),
            1 => Some(Self::LevelChange),
            2 => Some(Self::Generic),
            _ => None,
        }
    }
}

impl FromRaw<u64> for NSHapticFeedbackPerformanceTime {
    fn from_raw(value: u64) -> Option<Self> {
        match value {
            0 => Some(Self::Default),
            1 => Some(Self::Now),
            2 => Some(Self::DrawCompleted),
            _ => None,
        }
    }
}

unsafe fn _test_alignment() {
    let filter = NSAlignmentFeedbackFilter::new();

    let tokens = vec![
        filter.alignmentFeedbackTokenForHorizontalMovementInView_previousX_alignedX_defaultX(
            None,
            0.into(),
            0.into(),
            0.into(),
        ),
        filter.alignmentFeedbackTokenForVerticalMovementInView_previousY_alignedY_defaultY(
            None,
            0.into(),
            0.into(),
            0.into(),
        ),
        filter.alignmentFeedbackTokenForMovementInView_previousPoint_alignedPoint_defaultPoint(
            None,
            NSPoint::new(0.0, 0.0),
            NSPoint::new(0.0, 0.0),
            NSPoint::new(0.0, 0.0),
        ),
    ]
    .into_iter()
    .filter_map(|token| token)
    .collect::<Vec<_>>();

    let tokens_arr = NSArray::from_vec(tokens);
    filter.performFeedback_performanceTime(&tokens_arr, NSHapticFeedbackPerformanceTime::Now);
}
