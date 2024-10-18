use std::path::{Path, PathBuf};
use std::process::{Output, Stdio};

use bytes::Buf;
use tokio::io::AsyncWriteExt;
use tokio::process::Command;

use crate::built_info::{PKG_NAME, PKG_VERSION};
use crate::dependencies::IMDL;
use crate::errors::{AppError, OutputHandler};
use crate::imdl::TorrentSummary;
use crate::verify::SourceRule;
use crate::verify::SourceRule::IncorrectHash;

pub struct ImdlCommand;

impl ImdlCommand {
    #[allow(clippy::uninlined_format_args)]
    /// Create a torrent
    pub async fn create(
        content_dir: &Path,
        output_path: &Path,
        announce_url: String,
        source: String,
    ) -> Result<Output, AppError> {
        let output = Command::new(IMDL)
            .arg("torrent")
            .arg("create")
            .arg(content_dir.to_string_lossy().to_string())
            .arg("--private")
            .arg("--announce")
            .arg(announce_url)
            .arg("--comment")
            .arg(format!("Created with {} v{}", PKG_NAME, PKG_VERSION))
            .arg("--source")
            .arg(source.to_uppercase())
            .arg("--output")
            .arg(output_path.to_string_lossy().to_string())
            .arg("--no-created-by")
            .arg("--force")
            .output()
            .await
            .or_else(|e| AppError::command(e, "execute create torrent", IMDL))?;
        OutputHandler::execute(output, "create torrent", "IMDL")
    }

    /// Get a summary of the torrent file.
    pub async fn show(path: &Path) -> Result<TorrentSummary, AppError> {
        let output = Command::new(IMDL)
            .arg("torrent")
            .arg("show")
            .arg("--json")
            .arg(path)
            .output()
            .await
            .or_else(|e| AppError::command(e, "execute read torrent", IMDL))?;
        let output = OutputHandler::execute(output, "read torrent", "IMDL")?;
        let reader = output.stdout.reader();
        serde_json::from_reader(reader)
            .or_else(|e| AppError::json(e, "deserialize torrent"))
    }

    /// Verify files match the torrent metadata.
    pub async fn verify(
        torrent_file: &PathBuf,
        directory: &PathBuf,
    ) -> Result<Option<SourceRule>, AppError> {
        let output = Command::new(IMDL)
            .arg("torrent")
            .arg("verify")
            .arg("--content")
            .arg(directory)
            .arg(torrent_file)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await
            .or_else(|e| AppError::command(e, "execute verify torrent", IMDL))?;
        if output.status.success() {
            Ok(None)
        } else {
            let details = String::from_utf8(output.stderr).unwrap_or_default();
            Ok(Some(IncorrectHash(details)))
        }
    }

    /// Verify files match the torrent metadata.
    pub async fn verify_from_buffer(
        buffer: &[u8],
        directory: &PathBuf,
    ) -> Result<Vec<SourceRule>, AppError> {
        let mut child = Command::new(IMDL)
            .arg("torrent")
            .arg("verify")
            .arg("--content")
            .arg(directory)
            .arg("-")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .or_else(|e| AppError::command(e, "execute verify torrent", IMDL))?;
        let mut stdin = child.stdin.take().expect("stdin should be available");
        stdin
            .write_all(buffer)
            .await
            .or_else(|e| AppError::command(e, "writing buffer to verify torrent", IMDL))?;
        drop(stdin);
        let output = child
            .wait_with_output()
            .await
            .or_else(|e| AppError::command(e, "get output of verify torrent", IMDL))?;
        if output.status.success() {
            Ok(Vec::new())
        } else {
            let details = String::from_utf8(output.stderr).unwrap_or_default();
            Ok(vec![IncorrectHash(details)])
        }
    }
}
