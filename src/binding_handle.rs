// Credit: tauri/tray-icon
use std::{convert::Infallible, str::FromStr};

/// An unique id that is associated with a alignment feedbackfilter.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Default,
    Hash,
    serde::Serialize,
    serde::Deserialize,
)]
pub struct AlignmentFeedbackId(pub String);

impl AlignmentFeedbackId {
    /// Create a new alignment feedback filter id.
    pub fn new<S: AsRef<str>>(id: S) -> Self {
        Self(id.as_ref().to_string())
    }
}

impl AsRef<str> for AlignmentFeedbackId {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl<T: ToString> From<T> for AlignmentFeedbackId {
    fn from(value: T) -> Self {
        Self::new(value.to_string())
    }
}

impl FromStr for AlignmentFeedbackId {
    type Err = Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(Self::new(s))
    }
}

impl PartialEq<&str> for AlignmentFeedbackId {
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}

impl PartialEq<&str> for &AlignmentFeedbackId {
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}

impl PartialEq<String> for AlignmentFeedbackId {
    fn eq(&self, other: &String) -> bool {
        self.0 == *other
    }
}

impl PartialEq<String> for &AlignmentFeedbackId {
    fn eq(&self, other: &String) -> bool {
        self.0 == *other
    }
}

impl PartialEq<&String> for AlignmentFeedbackId {
    fn eq(&self, other: &&String) -> bool {
        self.0 == **other
    }
}

impl PartialEq<&AlignmentFeedbackId> for AlignmentFeedbackId {
    fn eq(&self, other: &&AlignmentFeedbackId) -> bool {
        other.0 == self.0
    }
}

#[cfg(test)]
mod test {
    use crate::binding_handle::AlignmentFeedbackId;

    #[test]
    fn is_eq() {
        assert_eq!(AlignmentFeedbackId::new("t"), "t",);
        assert_eq!(AlignmentFeedbackId::new("t"), String::from("t"));
        assert_eq!(AlignmentFeedbackId::new("t"), &String::from("t"));
        assert_eq!(AlignmentFeedbackId::new("t"), AlignmentFeedbackId::new("t"));
        assert_eq!(
            AlignmentFeedbackId::new("t"),
            &AlignmentFeedbackId::new("t")
        );
        assert_eq!(
            &AlignmentFeedbackId::new("t"),
            &AlignmentFeedbackId::new("t")
        );
        assert_eq!(AlignmentFeedbackId::new("t").as_ref(), "t");
    }
}
