use std::{fs::File, io::{self, Write}, process::exit};

use args::BinauralArgs;
use clap::Parser;
use tone::Tone;
use wav::{Wav, WavMetadata};

mod args;
mod tone;

/// A safe enviroment for main to throw io errors for main to handle.
fn main_io_handle() -> io::Result<()> {
    let BinauralArgs {
        sample_rate,
        base_freq: base_frequency,
        diff_freq: diff_frequency,
        duration: duration_in_seconds,
        output
    } = BinauralArgs::parse();

    let tones = Tone::new(
        sample_rate, 
        duration_in_seconds, 
        base_frequency as f64, 
        diff_frequency as f64
    ).generate_tones();

    let metadata = WavMetadata::new((
        sample_rate, 
        tones[0].len() as u32
    ));

    let wav_bytes = Wav::new(metadata, tones).to_bytes();

    let mut wav_file = File::create(output)?;

    _ = wav_file.write(&wav_bytes);

    Ok(())
}

fn main() {
    _ = main_io_handle().inspect_err(|e| {
        match e.kind() {
            e => eprintln!("UNCAUGHT ERROR: {e:?}"),
        };
        eprintln!("error: {}", e.to_string());
        exit(1)
    })
}
