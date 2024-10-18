use crate::errors::AppError;
use crate::queue::TimeStamp;
use crate::verify::SourceRule;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct VerifyStatus {
    success: bool,
    violations: Vec<SourceRule>,
    error: Option<AppError>,
    completed: TimeStamp,
}
