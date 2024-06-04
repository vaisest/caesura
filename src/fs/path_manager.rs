use std::path::PathBuf;

use di::{injectable, Ref};

use crate::formats::{TargetFormat, TargetFormatProvider};
use crate::fs::FlacFile;
use crate::naming::{SourceName, TargetName};
use crate::options::SharedOptions;
use crate::source::Source;

const SPECTROGRAM_DIR_NAME: &str = "spectrograms";
const TORRENT_DIR_NAME: &str = "torrents";
const TRANSCODE_DIR_NAME: &str = "transcodes";

#[injectable]
pub struct PathManager {
    shared_options: Ref<SharedOptions>,
    targets: Ref<TargetFormatProvider>,
}

impl PathManager {
    #[must_use]
    pub fn get_output_dir(&self) -> PathBuf {
        self.shared_options
            .output
            .clone()
            .expect("Option should be set")
    }

    #[must_use]
    pub fn get_spectrogram_dir(&self, source: &Source) -> PathBuf {
        self.get_output_dir()
            .join(SourceName::get_escaped(source))
            .join(SPECTROGRAM_DIR_NAME)
    }

    #[must_use]
    pub fn get_transcode_dir(&self, source: &Source) -> PathBuf {
        self.get_output_dir()
            .join(SourceName::get_escaped(source))
            .join(TRANSCODE_DIR_NAME)
    }

    #[must_use]
    pub fn get_transcode_target_dir(&self, source: &Source, target: &TargetFormat) -> PathBuf {
        self.get_transcode_dir(source)
            .join(TargetName::get(source, target))
    }

    #[must_use]
    pub fn get_transcode_path(
        &self,
        source: &Source,
        target: &TargetFormat,
        flac: &FlacFile,
    ) -> PathBuf {
        let extension = target.get_file_extension();
        let filename = flac.file_name.clone() + "." + extension.as_str();
        self.get_transcode_target_dir(source, target)
            .join(&flac.sub_dir)
            .join(filename)
    }

    #[must_use]
    pub fn get_max_transcode_sub_path(&self, source: &Source, flac: &FlacFile) -> String {
        let mut targets = self.targets.get(source.format, &source.existing);
        targets.sort();
        if targets.is_empty() {
            return "".to_owned();
        }
        let target = targets.last().expect("Should contain at least 1");
        let filename = flac.file_name.clone();
        let extension = target.get_file_extension();
        PathBuf::from(TargetName::get(source, target))
            .join(&flac.sub_dir)
            .join(format!("{filename}.{extension}"))
            .to_string_lossy()
            .to_string()
    }

    #[must_use]
    pub fn get_torrent_dir(&self, source: &Source) -> PathBuf {
        self.get_output_dir()
            .join(SourceName::get_escaped(source))
            .join(TORRENT_DIR_NAME)
    }

    #[must_use]
    pub fn get_torrent_path(&self, source: &Source, target: &TargetFormat) -> PathBuf {
        let filename = TargetName::get(source, target) + ".torrent";
        self.get_torrent_dir(source).join(filename)
    }
}
