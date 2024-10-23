use crate::errors::{AppError, OutputHandler};
use crate::transcode::{Decode, Encode, Resample, Variant};
use audiotags::{AudioTagWrite, Id3v2Tag};
use log::{trace, warn};
use std::fs::create_dir_all;
use std::process::Stdio;
use tokio::join;

pub struct TranscodeJob {
    pub id: String,
    pub variant: Variant,
    pub tags: Option<Id3v2Tag>,
}

impl TranscodeJob {
    pub async fn execute(self) -> Result<(), AppError> {
        let output_path = match &self.variant {
            Variant::Transcode(_, encode) => encode.output.clone(),
            Variant::Resample(resample) => resample.output.clone(),
        };
        let output_dir = output_path
            .parent()
            .expect("output path should have a parent");
        create_dir_all(output_dir)
            .or_else(|e| AppError::io(e, "create transcode output directory"))?;
        match self.variant {
            Variant::Transcode(decode, encode) => execute_transcode(decode, encode).await?,
            Variant::Resample(resample) => execute_resample(resample).await?,
        };
        if let Some(mut tags) = self.tags {
            tags.write_to_path(output_path.to_string_lossy().as_ref())
                .or_else(|e| AppError::tag(e, "write tags to file"))?;
        }
        Ok(())
    }
}

async fn execute_transcode(decode: Decode, encode: Encode) -> Result<(), AppError> {
    let decode_info = decode.to_info();
    let encode_info = encode.to_info();
    trace!("Executing transcode: {decode_info} | {encode_info}");
    let decode_program = decode_info.program.clone();
    let mut decode_command = decode_info
        .to_command()
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .or_else(|e| AppError::command(e, "spawn decode", &decode_program))?;
    let pipe: Stdio = decode_command
        .stdout
        .take()
        .expect("should be able to take stdout")
        .try_into()
        .expect("should be able to convert stdout to pipe");
    let encode_program = encode_info.program.clone();
    let encode_command = encode_info
        .to_command()
        .stdin(pipe)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .or_else(|e| AppError::command(e, "spawn decode", &encode_program))?;
    let (decode_result, encode_output) =
        join!(decode_command.wait(), encode_command.wait_with_output());
    let decode_exit = decode_result.or_else(|e| AppError::io(e, "wait for decode"))?;
    let encode_output = encode_output.or_else(|e| AppError::io(e, "wait for encode"))?;
    if !decode_exit.success() {
        warn!("Decode was not successful: {decode_exit}");
    }
    OutputHandler::execute(encode_output, "execute resample job", "transcode")?;
    Ok(())
}

async fn execute_resample(resample: Resample) -> Result<(), AppError> {
    let info = resample.to_info();
    trace!("Executing resample: {info}");
    let program = info.program.clone();
    let output = info
        .to_command()
        .output()
        .await
        .or_else(|e| AppError::command(e, "execute resample job", &program))?;
    OutputHandler::execute(output, "execute resample job", "transcode")?;
    Ok(())
}
