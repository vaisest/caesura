use std::path::PathBuf;

use crate::formats::TargetFormat;
use crate::fs::FlacFile;
use crate::imdl::ImdlCommand;
use crate::naming::{SpectrogramName, TranscodeName};
use crate::options::SharedOptions;
use crate::source::Source;
use di::{injectable, Ref};
use rogue_logging::Error;

#[injectable]
pub struct PathManager {
    shared_options: Ref<SharedOptions>,
}

impl PathManager {
    #[must_use]
    pub fn get_output_dir(&self) -> PathBuf {
        self.shared_options
            .output
            .clone()
            .expect("output should be set")
    }

    #[must_use]
    pub fn get_spectrogram_dir(&self, source: &Source) -> PathBuf {
        self.get_output_dir()
            .join(SpectrogramName::get(&source.metadata))
    }

    #[must_use]
    pub fn get_transcode_target_dir(&self, source: &Source, target: TargetFormat) -> PathBuf {
        self.get_output_dir()
            .join(TranscodeName::get(&source.metadata, target))
    }

    #[must_use]
    pub fn get_transcode_path(
        &self,
        source: &Source,
        target: TargetFormat,
        flac: &FlacFile,
    ) -> PathBuf {
        let extension = target.get_file_extension();
        let filename = flac.file_name.clone() + "." + extension.as_str();
        self.get_transcode_target_dir(source, target)
            .join(&flac.sub_dir)
            .join(filename)
    }

    #[must_use]
    pub fn get_torrent_path(
        &self,
        source: &Source,
        target: TargetFormat,
        include_indexer: bool,
    ) -> PathBuf {
        let mut filename = TranscodeName::get(&source.metadata, target);
        if include_indexer {
            let indexer = self
                .shared_options
                .indexer
                .clone()
                .expect("indexer should be set");
            filename.push('.');
            filename.push_str(&indexer);
        }
        filename.push_str(".torrent");
        self.get_output_dir().join(filename)
    }

    /// Get the *torrent path with suffix* if it exists.
    ///
    /// Example `path/to/Artist - Album [2012] [WEB FLAC].abc.torrent`
    ///
    /// Returns `None` if the path does not exist or an existing torrent can't be copied or
    /// re-created with the indexer suffix.
    ///
    /// Returns the *torrent path with suffix* if it already exists.
    ///
    /// Or attempt to copy or re-create from an existing torrent file
    /// (`path/to/Artist - Album [2012] [WEB FLAC].torrent`).
    ///
    /// Returns the *torrent path with suffix* if duplication is successful, else `None`
    pub async fn get_or_duplicate_existing_torrent_path(
        &self,
        source: &Source,
        target: TargetFormat,
    ) -> Result<Option<PathBuf>, Error> {
        let path_with_indexer = self.get_torrent_path(source, target, true);
        if path_with_indexer.is_file() {
            return Ok(Some(path_with_indexer));
        }
        let path_without_indexer = self.get_torrent_path(source, target, false);
        if !path_without_indexer.is_file() {
            return Ok(None);
        }
        let transcode_dir = self.get_transcode_target_dir(source, target);
        let announce_url = self
            .shared_options
            .announce_url
            .clone()
            .expect("announce should be set");
        let indexer = self
            .shared_options
            .indexer
            .clone()
            .expect("indexer should be set")
            .to_lowercase();
        let success = ImdlCommand::duplicate_torrent(
            &path_without_indexer,
            &path_with_indexer,
            &transcode_dir,
            announce_url,
            indexer,
        )
        .await?;
        if success {
            Ok(Some(path_with_indexer))
        } else {
            Ok(None)
        }
    }
}
