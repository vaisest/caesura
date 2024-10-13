use std::path::PathBuf;

use di::{injectable, Ref};

use crate::formats::{TargetFormat, TargetFormatProvider};
use crate::fs::FlacFile;
use crate::naming::{SpectrogramName, TranscodeName};
use crate::options::SharedOptions;
use crate::source::Source;

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
            .expect("output should be set")
    }

    #[must_use]
    pub fn get_spectrogram_dir(&self, source: &Source) -> PathBuf {
        self.get_output_dir()
            .join(SpectrogramName::get(&source.metadata))
    }

    #[must_use]
    pub fn get_transcode_target_dir(&self, source: &Source, target: &TargetFormat) -> PathBuf {
        self.get_output_dir()
            .join(TranscodeName::get(&source.metadata, target))
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
            return String::new();
        }
        let target = targets.last().expect("Should contain at least 1");
        let filename = flac.file_name.clone();
        let extension = target.get_file_extension();
        PathBuf::from(TranscodeName::get(&source.metadata, target))
            .join(&flac.sub_dir)
            .join(format!("{filename}.{extension}"))
            .to_string_lossy()
            .to_string()
    }

    #[must_use]
    pub fn get_torrent_path(&self, source: &Source, target: &TargetFormat) -> PathBuf {
        let filename = TranscodeName::get(&source.metadata, target) + ".torrent";
        self.get_output_dir().join(filename)
    }
}
