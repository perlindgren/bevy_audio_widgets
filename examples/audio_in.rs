//! Records a WAV file (roughly 3 seconds long) using the default input device and config.
//!
//! The input data is recorded to "$CARGO_MANIFEST_DIR/recorded.wav".

mod common;
use common::audio_in;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{/* FromSample, */ Sample};

use crate::common::audio_in::parse_input;
// use std::sync::{Arc, Mutex};

fn main() -> Result<(), anyhow::Error> {
    let (opt, stream) = parse_input(write_input_data)?;

    stream.play()?;

    // Let recording go for roughly three seconds.
    std::thread::sleep(std::time::Duration::from_secs(opt.duration));
    drop(stream);

    Ok(())
}

fn write_input_data<T: cpal::Sample>(input: &[T]) {
    //     println!("Received input data: {:?}", input.len());
    //     for sample in input.iter() {
    //         let sample: i16 = cpal::Sample::to_i16(sample);
    //         //     //     writer.write_sample(sample).ok();
    //     }
}

// fn write_input_data(input: &[i16]) {
//     println!("Received input data: {:?}", input.len());
//     for sample in input.iter() {
//         let sample: i16 = *sample;
//         //     //     writer.write_sample(sample).ok();
//     }
// }
