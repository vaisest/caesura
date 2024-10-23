use crate::errors::AppError;
use claxon::metadata::StreamInfo;

#[must_use]
pub fn is_resample_required(info: &StreamInfo) -> bool {
    info.sample_rate > 48000 || info.bits_per_sample > 16
}

pub fn get_resample_rate(info: &StreamInfo) -> Result<u32, AppError> {
    if info.sample_rate % 44100 == 0 {
        Ok(44100)
    } else if info.sample_rate % 48000 == 0 {
        Ok(48000)
    } else {
        AppError::explained("get sample rate", "invalid sample rate".to_owned())
    }
}

/// Get the average bit rate in bits per second.
///
/// Returns `None` if StreamInfo.samples is None.
#[allow(
    clippy::cast_precision_loss,
    clippy::cast_lossless,
    clippy::cast_sign_loss
)]
pub fn get_average_bit_rate(info: &StreamInfo) -> Option<u32> {
    let total_samples = info.samples?;
    let total_bits = total_samples * info.bits_per_sample as u64 * info.channels as u64;
    let duration_seconds = total_samples as f64 / info.sample_rate as f64;
    let bit_rate = total_bits as f64 / duration_seconds;
    let bit_rate = bit_rate.round() as u32;
    Some(bit_rate)
}

/// Get the duration in seconds.
///
/// Returns `None` if StreamInfo.samples is None.
#[allow(clippy::cast_precision_loss, clippy::cast_sign_loss)]
pub fn get_duration(info: &StreamInfo) -> Option<u32> {
    let seconds = info.samples? as f64 / f64::from(info.sample_rate);
    Some(seconds.round() as u32)
}
