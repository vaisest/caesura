use std::sync::Arc;

use colored::Colorize;
use di::injectable;
use log::trace;
use tokio::sync::Semaphore;

use crate::jobs::{Status, Subscriber};
use crate::logging::Colors;
use crate::options::RunnerOptions;

/// Log all [Command] status updates to the console by subscribing to
/// [`CommandRunner`] as a [Subscriber].
pub struct DebugSubscriber {
    pub semaphore: Arc<Semaphore>,
    pub cpus: u16,
}

#[injectable]
impl DebugSubscriber {
    pub fn new(options: Arc<RunnerOptions>, semaphore: Arc<Semaphore>) -> Self {
        let cpus = options.cpus.expect("Options should be set");
        Self { semaphore, cpus }
    }
}

impl Subscriber for DebugSubscriber {
    /// Called when a new scope is started.
    fn start(&self, _scope_id: &str) {}

    /// Called when a scope is finished.
    fn finish(&self, _scope_id: &str) {}

    /// Called when the status of a job changes.
    fn update(&self, job_id: &str, status: Status) {
        let available = self.semaphore.available_permits();
        let in_use = self.cpus - available as u16;
        let total = self.cpus;
        trace!(
            "{:>9} {} {}",
            status.to_string().bold(),
            job_id,
            format!("[Active:{in_use:>3}/{total}]").dark_gray()
        );
    }
}
