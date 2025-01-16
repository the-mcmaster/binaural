use crate::{Wav, WavHeader, WavMetadata};

impl WavMetadata {
    pub fn new<T: Into<WavMetadata>>(data: T) -> Self {
        data.into()
    }

    fn default_blank() -> Self {
        WavMetadata {
            sample_rate: 44100,
            number_of_channels: 2,
            number_of_samples: 0 // << this is the "blank"
        }
    }
}
impl From<u32> for WavMetadata {
    fn from(number_of_samples: u32) -> Self {
        let WavMetadata {
            sample_rate,
            number_of_channels,
            ..
        } = WavMetadata::default_blank();

        WavMetadata {
            sample_rate,
            number_of_channels,
            number_of_samples
        }
    }
}
impl From<(u32, u32)> for WavMetadata {
    fn from((sample_rate, number_of_samples): (u32, u32)) -> Self {
        let WavMetadata {
            number_of_channels,
            ..
        } = WavMetadata::default_blank();

        WavMetadata {
            sample_rate,
            number_of_channels,
            number_of_samples
        }
    }
}
impl From<(u16, u32)> for WavMetadata {
    fn from((number_of_channels, number_of_samples): (u16, u32)) -> Self {
        let WavMetadata {
            sample_rate,
            ..
        } = WavMetadata::default_blank();

        WavMetadata {
            sample_rate,
            number_of_channels,
            number_of_samples
        }
    }
}
impl From<(u32, u16, u32)> for WavMetadata {
    fn from((sample_rate, number_of_channels, number_of_samples): (u32, u16, u32)) -> Self {
        WavMetadata {
            sample_rate,
            number_of_channels,
            number_of_samples
        }
    }
}

impl WavHeader {
    pub fn new(metadata: WavMetadata) -> Self {WavHeader { metadata }}

    /// The number of bytes needed to represent the RIFF marker.
    const RIFF_MARKER_BYTES: usize = 4;

    /// The number of bytes needed to represent the overall file size in bytes.
    const FILE_SIZE_BYTES: usize = 4;

    /// The number of bytes needed to represent the WAVE marker.
    const WAVE_MARKER_BYTES: usize = 4;

    /// The number of bytes needed to represent the format marker.
    const FORMAT_MARKER_BYTES: usize = 4;

    /// The number of bytes needed to represent the format length.
    const FORMAT_LENGTH_BYTES: usize = 4;

    /// The number of bytes needed to represent the format type.
    const FORMAT_TYPE_BYTES: usize = 2;

    /// The number of bytes needed to represent the number of channels.
    const NUMBER_OF_CHANNELS_BYTES: usize = 2;

    /// The number of bytes needed to represent the sample rate.
    const SAMPLE_RATE_BYTES: usize = 4;

    /// The number of bytes needed to represent the number of bytes per second.
    const BYTES_PER_SECOND_BYTES: usize = 4;

    /// The number of bytes needed to represent the block alignment.
    const BLOCK_ALIGNMENT_BYTES: usize = 2;

    /// The number of bytes needed to represent number of bits per sample.
    const BITS_PER_SAMPLE_BYTES: usize = 2;

    /// The number of bytes needed to represent the data marker.
    const DATA_MARKER_BYTES: usize = 4;

    /// The number of bytes needed to represent the data size.
    const DATA_SIZE_BYTES: usize = 4;

    /// The number of bytes needed to represent the entire header.
    const TOTAL_HEADER_SIZE_BYTES: usize = 
        Self::RIFF_MARKER_BYTES + Self::FILE_SIZE_BYTES + Self::WAVE_MARKER_BYTES +
        Self::FORMAT_MARKER_BYTES + Self::FORMAT_LENGTH_BYTES + Self::FORMAT_TYPE_BYTES +
        Self::NUMBER_OF_CHANNELS_BYTES + Self::SAMPLE_RATE_BYTES + Self::BYTES_PER_SECOND_BYTES +
        Self::BLOCK_ALIGNMENT_BYTES + Self::BITS_PER_SAMPLE_BYTES + Self::DATA_MARKER_BYTES + Self::DATA_SIZE_BYTES;

    pub fn to_bytes(&self) -> Vec<u8> {
        const BYTES_PER_SAMPLE: u16 = Wav::BITS / 8;

        let mut buffer = vec![];

        let bytes_per_sample_total = BYTES_PER_SAMPLE as u16 * self.metadata.number_of_channels;
        let data_size = bytes_per_sample_total as u32 * self.metadata.number_of_samples;
        let bytes_per_second = bytes_per_sample_total as u32 * self.metadata.sample_rate;
        let bytes_per_second_even = if bytes_per_second % 2 == 0 { bytes_per_second } else { bytes_per_second + 1 };
        let file_size = (Self::TOTAL_HEADER_SIZE_BYTES - Self::RIFF_MARKER_BYTES - Self::FORMAT_LENGTH_BYTES) as u32 + data_size;

        // I AM NOT SURE HOW I FEEL ABOUT THIS CODE.
        // maybe better as a macro??????
        // this is very experimental for me
        buffer.extend("RIFF".as_bytes().iter().chain(
        Self::u32_bytes(file_size).iter().chain(
        "WAVE".as_bytes().iter().chain(
        "fmt ".as_bytes().iter().chain(
        Self::u32_bytes(16).iter().chain(
        Self::u16_bytes(1).iter().chain(
        Self::u16_bytes(self.metadata.number_of_channels).iter().chain(
        Self::u32_bytes(self.metadata.sample_rate).iter().chain(
        Self::u32_bytes(bytes_per_second_even).iter().chain(
        Self::u16_bytes(bytes_per_sample_total).iter().chain(
        Self::u16_bytes(Wav::BITS).iter().chain(
        "data".as_bytes().iter().chain(Self::u32_bytes(data_size).iter())))))))))))));

        return buffer

    }

    fn u32_bytes(n: u32) -> [u8; 4] {
        [
            ((n & (0xF << 24)) >> 24) as u8,
            ((n & (0xF << 16)) >> 16) as u8,
            ((n & (0xF <<  8)) >> 8 ) as u8,
            ((n & (0xF <<  0)) >> 0 ) as u8
        ]
    }

    fn u16_bytes(n: u16) -> [u8; 2] {
        [
            ((n & (0xF <<  8)) >> 8 ) as u8,
            ((n & (0xF <<  0)) >> 0 ) as u8
        ]
    }
}

impl Wav {
    pub fn new(metadata: WavMetadata, channel_data: Vec<Vec<f64>>) -> Self {
        Wav {
            metadata,
            channel_data
        }
    }

    const BITS: u16 = (u16::BITS) as u16;

    fn percent_to_int(percent: f64) -> i32 {
        let max_value = 2.0_f64.powi(Self::BITS as i32);

        let adj_value = (percent * ((max_value / 2.0) - 1.0)).floor();
        // this can probably be handled better...
        let folded_value = if adj_value >= max_value {max_value} else if adj_value < 0.0 {-adj_value} else {adj_value};

        return unsafe{ folded_value.to_int_unchecked() }
    }

    fn data_to_bytes(&self) -> Vec<u8> {
        let mut buffer: Vec<u8> = vec![];

        for sample in 0..self.metadata.number_of_samples {
            for channel in &self.channel_data {
                if (sample as usize) < channel.len() {
                    let byte = Self::percent_to_int(channel[sample as usize]);
                } else {
                    buffer.push(0u8);
                }
            }
        }

        if buffer.len() % 2 == 1 {
            buffer.push(0u8);
        }

        return buffer
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buffer = vec![];

        let header_bytes = WavHeader::new(self.metadata).to_bytes();
        let data_bytes = self.data_to_bytes();

        buffer.extend(header_bytes);
        buffer.extend(data_bytes);

        return buffer
    }
}