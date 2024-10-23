use crate::transcode::decode::Decode;
use crate::transcode::encode::Encode;
use crate::transcode::resample::Resample;

pub enum Variant {
    Transcode(Decode, Encode),
    Resample(Resample),
}
