use std::fs::create_dir_all;
use std::path::PathBuf;
use std::process::Stdio;

use audiotags::{AudioTagWrite, Id3v2Tag};
use colored::Colorize;
use log::*;
use tokio::io::AsyncWriteExt;

use crate::errors::{AppError, OutputHandler};
use crate::transcode::CommandFactory;

pub struct TranscodeJob {
    pub id: String,
    pub output_path: PathBuf,
    pub commands: Vec<CommandFactory>,
    pub tags: Option<Id3v2Tag>,
}

impl TranscodeJob {
    pub async fn execute(self) -> Result<(), AppError> {
        let output_dir = self
            .output_path
            .parent()
            .expect("output path should have a parent");
        create_dir_all(output_dir)
            .or_else(|e| AppError::io(e, "create transcode output directory"))?;
        let mut buffer = vec![];
        for factory in self.commands {
            let command = format!(
                "{} \"{}\"",
                factory.program.clone(),
                factory.args.clone().join("\" \"")
            );
            trace!("{} {}", "Executing".bold(), command);
            let mut child = factory
                .create()
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .or_else(|e| AppError::io(e, "execute transcode job"))?;
            if !buffer.is_empty() {
                let mut stdin = child.stdin.take().expect("stdin should be available");
                stdin
                    .write_all(&buffer)
                    .await
                    .expect("Should be able to write to std in");
                drop(stdin);
            }
            let output = child
                .wait_with_output()
                .await
                .expect("Child should produce an output");
            let output = OutputHandler::execute(output, "transcode", "transcode")?;
            buffer = output.stdout;
        }
        if let Some(tags) = self.tags {
            let mut tags = tags;
            tags.write_to_path(self.output_path.to_string_lossy().as_ref())
                .or_else(|e| AppError::tag(e, "write tags to file"))?;
        }
        Ok(())
    }
}
