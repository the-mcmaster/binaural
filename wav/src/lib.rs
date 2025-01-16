#![allow(unused)]

mod wav_impls;

#[derive(Clone)]
pub struct Wav {
    metadata: WavMetadata,
    channel_data: Vec<Vec<f64>>
}

#[derive(Clone, Copy)]
pub struct WavMetadata {
    sample_rate: u32,
    number_of_channels: u16,
    number_of_samples: u32
}

#[derive(Clone, Copy)]
pub struct WavHeader {
    metadata: WavMetadata
}