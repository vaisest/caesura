use std::process::exit;
use std::sync::Arc;

use colored::Colorize;
use di::{singleton_as_self, Injectable, Mut, Ref, RefMut, ServiceCollection};
use log::error;
use tokio::sync::Semaphore;
use tokio::task::JoinSet;

use crate::api::{Api, ApiFactory};
use crate::errors::AppError;
use crate::formats::TargetFormatProvider;
use crate::fs::PathManager;
use crate::hosting::Host;
use crate::jobs::{DebugSubscriber, JobRunner, ProgressBarSubscriber, Publisher};
use crate::logging::{Logger, Trace};
use crate::options::{FileOptions, Options, OptionsProvider, RunnerOptions, SharedOptions, SpectrogramOptions, TargetOptions, UploadOptions, VerifyOptions};
use crate::source::SourceProvider;
use crate::spectrogram::{SpectrogramCommand, SpectrogramJobFactory};
use crate::transcode::{AdditionalJobFactory, TranscodeCommand, TranscodeJobFactory};
use crate::upload::UploadCommand;
use crate::verify::VerifyCommand;

pub struct HostBuilder {
    pub services: ServiceCollection,
}

impl Default for HostBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl HostBuilder {
    #[must_use]
    pub fn new() -> HostBuilder {
        let mut this = HostBuilder {
            services: ServiceCollection::new(),
        };
        this.services
            // Add options
            .add(OptionsProvider::singleton())
            .add(FileOptions::singleton())
            .add(RunnerOptions::singleton())
            .add(SharedOptions::singleton())
            .add(SpectrogramOptions::singleton())
            .add(TargetOptions::singleton())
            .add(UploadOptions::singleton())
            .add(VerifyOptions::singleton())
            // Add main services
            .add(Logger::singleton())
            .add(PathManager::transient())
            .add(SourceProvider::transient().as_mut())
            .add(ApiFactory::transient())
            .add(Api::singleton().as_mut())
            .add(JobRunner::transient())
            .add(Publisher::transient())
            .add(DebugSubscriber::transient())
            .add(ProgressBarSubscriber::transient())
            .add(TargetFormatProvider::transient())
            // Add transcode services
            .add(TranscodeCommand::transient())
            .add(TranscodeJobFactory::transient())
            .add(AdditionalJobFactory::transient())
            // Add spectrogram services
            .add(SpectrogramCommand::transient())
            .add(SpectrogramJobFactory::transient())
            .add(singleton_as_self().from(|provider| {
                let options = provider.get_required::<RunnerOptions>();
                let cpus = options.get_value(|x| x.cpus) as usize;
                Arc::new(Semaphore::new(cpus))
            }))
            .add(singleton_as_self().from(|_| {
                let set: JoinSet<Result<(), AppError>> = JoinSet::new();
                RefMut::new(Mut::new(set))
            }))
            // Add upload services
            .add(UploadCommand::transient().as_mut())
            // Add verify services
            .add(VerifyCommand::transient().as_mut());
        this
    }

    #[must_use]
    pub fn with_options<T: Options + 'static>(&mut self, options: T) -> &mut Self {
        self.services
            .add(singleton_as_self().from(move |_| Ref::new(options.clone())));
        self
    }

    #[must_use]
    pub fn build(&self) -> Host {
        match self.services.build_provider() {
            Ok(services) => Host::new(services),
            Err(error) => {
                Logger::init_new(Trace);
                error!("{} to build the application:", "Failed".red().bold());
                error!("{error}");
                exit(1)
            }
        }
    }
}
