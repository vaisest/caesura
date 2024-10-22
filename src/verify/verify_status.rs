use crate::queue::TimeStamp;
use crate::source::SourceIssue;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct VerifyStatus {
    pub verified: bool,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub issues: Vec<SourceIssue>,
    pub completed: TimeStamp,
}

impl VerifyStatus {
    pub fn new(issues: Vec<SourceIssue>) -> Self {
        Self {
            verified: issues.is_empty(),
            issues,
            completed: TimeStamp::now(),
        }
    }
}
