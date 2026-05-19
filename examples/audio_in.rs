//! Example of audio input with user-configurable options.
//! The raw sample data is printed to the console.

mod common;
// use common::audio_in;

use cpal::traits::StreamTrait;
// use cpal::{FromSample, Sample};

use common::audio_in::parse_input;

use crate::common::audio_in::open_input_stream;

fn main() -> Result<(), anyhow::Error> {
    let opt = parse_input();
    let stream = open_input_stream(&opt, write_input_data)?;

    stream.play()?;

    // Let recording go for roughly three seconds.
    std::thread::sleep(std::time::Duration::from_secs(opt.duration));
    drop(stream);

    Ok(())
}

fn write_input_data(input: &[f32]) {
    println!("Received input data: {:?}", input.len());
    for sample in input.iter() {
        println!("Sample: {}", sample);
    }
}
