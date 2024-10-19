use colored::Colorize;
use di::{injectable, Ref, RefMut};
use log::*;

use crate::api::Api;
use crate::errors::AppError;
use crate::formats::TargetFormatProvider;
use crate::fs::{Collector, PathManager};
use crate::imdl::imdl_command::ImdlCommand;
use crate::naming::{Shortener, SourceName};
use crate::options::verify_options::VerifyOptions;
use crate::options::{Options, SharedOptions};
use crate::source::*;
use crate::verify::tag_verifier::TagVerifier;
use crate::verify::verify_status::VerifyStatus;
use crate::verify::SourceRule::*;
use crate::verify::*;

/// Verify a FLAC source is suitable for transcoding.
#[injectable]
pub struct VerifyCommand {
    shared_options: Ref<SharedOptions>,
    verify_options: Ref<VerifyOptions>,
    source_provider: RefMut<SourceProvider>,
    api: RefMut<Api>,
    targets: Ref<TargetFormatProvider>,
    paths: Ref<PathManager>,
}

impl VerifyCommand {
    pub async fn execute_cli(&mut self) -> Result<bool, AppError> {
        if !self.shared_options.validate() || !self.verify_options.validate() {
            return Ok(false);
        }
        let source = self
            .source_provider
            .write()
            .expect("Source provider should be writeable")
            .get_from_options()
            .await?;
        let status = self.execute(&source).await;
        if status.verified {
            info!("{} {}", "Verified".bold(), source);
        } else {
            warn!("{} to verify {}", "Failed".bold(), source);
            for violation in status.violations {
                warn!("{violation}");
            }
        }
        Ok(status.verified)
    }

    pub async fn execute(&mut self, source: &Source) -> VerifyStatus {
        debug!("{} {}", "Verifying".bold(), source);
        Self::name_checks(source);
        let mut errors: Vec<SourceRule> = Vec::new();
        errors.append(&mut self.api_checks(source));
        errors.append(&mut self.flac_checks(source));
        errors.append(&mut self.hash_check(source).await);
        VerifyStatus::new(errors)
    }

    fn name_checks(source: &Source) {
        let sanitized = SourceName::get(&source.metadata);
        let unsanitized = SourceName::get_unsanitized(&source.metadata);
        if sanitized != unsanitized {
            debug!("Source name has been sanitized: {sanitized}");
        }
    }

    fn api_checks(&self, source: &Source) -> Vec<SourceRule> {
        let mut errors: Vec<SourceRule> = Vec::new();
        if source.group.category_name != "Music" {
            errors.push(Category {
                actual: source.group.category_name.clone(),
            });
        }
        if source.torrent.scene {
            errors.push(Scene);
        }
        if source.torrent.lossy_master_approved == Some(true) {
            errors.push(LossyMaster);
        }
        if source.torrent.lossy_web_approved == Some(true) {
            errors.push(LossyWeb);
        }
        if source.torrent.trumpable == Some(true) {
            errors.push(Trumpable);
        }
        let target_formats = self.targets.get(source.format, &source.existing);
        if target_formats.is_empty() {
            errors.push(Existing {
                formats: source.existing.clone(),
            });
        }
        errors
    }

    #[allow(clippy::cast_sign_loss, clippy::cast_possible_wrap)]
    fn flac_checks(&self, source: &Source) -> Vec<SourceRule> {
        if !source.directory.exists() || !source.directory.is_dir() {
            return vec![MissingDirectory {
                path: source.directory.clone(),
            }];
        }
        let flacs = Collector::get_flacs(&source.directory);
        if flacs.is_empty() {
            return vec![NoFlacs {
                path: source.directory.clone(),
            }];
        }
        let mut errors: Vec<SourceRule> = Vec::new();
        let max_target = self
            .targets
            .get_max_path_length(source.format, &source.existing);
        let mut too_long = false;
        for flac in flacs {
            if let Some(max_path) = max_target {
                let path = self.paths.get_transcode_path(source, max_path, &flac);
                let length = path.to_string_lossy().len() as isize;
                let excess = length - MAX_PATH_LENGTH;
                if excess > 0 {
                    let excess = excess as usize;
                    errors.push(Length { path, excess });
                    Shortener::suggest_track_name(&flac);
                    too_long = true;
                }
            }
            let tags = TagVerifier::execute(&flac, source)
                .unwrap_or(vec!["failed to retrieve tags".to_owned()]);
            if !tags.is_empty() {
                errors.push(MissingTags {
                    path: flac.path.clone(),
                    tags,
                });
            }
            for error in StreamVerifier::execute(&flac) {
                errors.push(error);
            }
        }
        if too_long {
            Shortener::suggest_album_name(source);
        }
        errors
    }

    async fn hash_check(&mut self, source: &Source) -> Vec<SourceRule> {
        if self
            .verify_options
            .no_hash_check
            .expect("no_hash_check should be set")
        {
            debug!("{} hash check due to settings", "Skipped".bold());
            return Vec::new();
        }
        let mut api = self.api.write().expect("API should be available");
        match api.get_torrent_file_as_buffer(source.torrent.id).await {
            Ok(buffer) => ImdlCommand::verify_from_buffer(&buffer, &source.directory)
                .await
                .unwrap_or_else(|e| {
                    vec![Error {
                        domain: "IMDL".to_owned(),
                        details: e.to_string(),
                    }]
                }),
            Err(e) => {
                vec![Error {
                    domain: "API".to_owned(),
                    details: e.to_string(),
                }]
            }
        }
    }
}
