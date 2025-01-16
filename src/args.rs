use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = "Shows the program usage.")]
pub struct BinauralArgs {
    /// The sample rate of the resulting audio file.
    #[clap(short='r', long, default_value_t = 44100)]
    pub sample_rate: u32,

    /// The base frequency to play on the left channel.
    #[clap(short, long)]
    pub base_freq: u32,
    
    /// The offset for the base frequency to play on the right channel.
    #[clap(short, long)]
    pub diff_freq: u32,
    
    /// The duration in seconds for the audio file.
    #[clap(short='t', long, default_value_t = 5)]
    pub duration: u32,
    
    /// The location of the output file.
    #[clap(short, long)]
    pub output: String
}