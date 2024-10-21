use crate::queue::TimeStamp;
use crate::verify::SourceRule;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct VerifyStatus {
    pub verified: bool,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub violations: Vec<SourceRule>,
    pub completed: TimeStamp,
}

impl VerifyStatus {
    pub fn new(violations: Vec<SourceRule>) -> Self {
        Self {
            verified: violations.is_empty(),
            violations,
            completed: TimeStamp::now(),
        }
    }
}
