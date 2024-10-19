use crate::queue::TimeStamp;
use crate::verify::SourceRule;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct VerifyStatus {
    pub verified: bool,
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
