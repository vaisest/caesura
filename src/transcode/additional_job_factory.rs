use di::{injectable, Ref};

use crate::formats::target_format::TargetFormat;
use crate::fs::{AdditionalFile, PathManager};
use crate::jobs::Job;
use crate::options::{FileOptions, Options};
use crate::source::Source;
use crate::transcode::AdditionalJob;

#[injectable]
pub struct AdditionalJobFactory {
    options: Ref<FileOptions>,
    paths: Ref<PathManager>,
}

impl AdditionalJobFactory {
    /// Create a [`AdditionalJob`] for each [`FlacFile`] in the [`Vec<FlacFile>`].
    #[must_use]
    pub fn create(
        &self,
        files: &[AdditionalFile],
        source: &Source,
        target: TargetFormat,
    ) -> Vec<Job> {
        let mut jobs = Vec::new();
        for (index, file) in files.iter().enumerate() {
            jobs.push(self.create_single(index, file, source, target));
        }
        jobs
    }

    /// Create a single [`AdditionalJob`] from a `flac_file`.
    fn create_single(
        &self,
        index: usize,
        file: &AdditionalFile,
        source: &Source,
        target: TargetFormat,
    ) -> Job {
        let id = format!("Additional {target:<7?}{index:>3}");
        let source_path = file.path.clone();
        let output_dir = self
            .paths
            .get_transcode_target_dir(source, &target)
            .join(&file.sub_dir);
        let output_path = output_dir
            .join(&file.file_name)
            .to_string_lossy()
            .into_owned();
        let output_dir = output_dir.to_string_lossy().into_owned();
        let extension = source_path
            .extension()
            .expect("Source has extension")
            .to_string_lossy()
            .into_owned();
        let compress_images = self.options.get_value(|x| x.compress_images);
        let max_file_size = self.options.get_value(|x| x.max_file_size);
        let max_pixel_size = self.options.get_value(|x| x.max_pixel_size);
        let quality = self.options.get_value(|x| x.jpg_quality);
        let png_to_jpg = self.options.get_value(|x| x.png_to_jpg);
        let hard_link = self.options.get_value(|x| x.hard_link);
        Job::Additional(AdditionalJob {
            id,
            source_path,
            output_dir,
            output_path,
            hard_link,
            compress_images,
            max_file_size,
            max_pixel_size,
            quality,
            png_to_jpg,
            extension,
        })
    }
}
