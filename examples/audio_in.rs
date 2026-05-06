//! Records a WAV file (roughly 3 seconds long) using the default input device and config.
//!
//! The input data is recorded to "$CARGO_MANIFEST_DIR/recorded.wav".

use clap::Parser;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{/* FromSample, */ Sample};
// use std::sync::{Arc, Mutex};

#[derive(Parser, Debug)]
#[command(version, about = "CPAL record_wav example", long_about = None)]
struct Opt {
    /// The audio device to use.
    #[arg(short, long)]
    device: Option<String>,

    /// How long to record, in seconds
    #[arg(long, default_value_t = 1)]
    duration: u64,

    /// Use the JACK host
    #[cfg(any(
        target_os = "linux",
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "netbsd"
    ))]
    #[arg(short, long)]
    #[allow(dead_code)]
    jack: bool,
}

fn main() -> Result<(), anyhow::Error> {
    let opt = Opt::parse();

    // Conditionally compile with jack if the feature is specified.
    #[cfg(any(
        target_os = "linux",
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "netbsd"
    ))]
    // Manually check for flags. Can be passed through cargo with -- e.g.
    // cargo run --release --example audio_in -- --jack
    let host = if opt.jack {
        cpal::host_from_id(
            cpal::available_hosts()
                .into_iter()
                .find(|id| *id == cpal::HostId::Jack)
                .expect("Only works on OSes where jack is available"),
        )
        .expect("jack host unavailable")
    } else {
        cpal::default_host()
    };

    #[cfg(not(any(
        target_os = "linux",
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "netbsd"
    )))]
    let host = cpal::default_host();

    // Set up the input device and stream with the default input config.
    let device = if let Some(device) = opt.device {
        let id = &device.parse().expect("failed to parse input device id");
        host.device_by_id(id)
    } else {
        host.default_input_device()
    }
    .expect("failed to find input device");

    println!("Input device: {}", device.id()?);

    let config = if device.supports_input() {
        device.default_input_config()
    } else {
        device.default_output_config()
    }
    .expect("Failed to get default input/output config");
    println!("Default input/output config: {config:?}");

    // A flag to indicate that recording is in progress.
    println!("Begin recording...");

    let err_fn = move |err| {
        eprintln!("an error occurred on stream: {err}");
    };

    let stream = match config.sample_format() {
        cpal::SampleFormat::I8 => device.build_input_stream(
            &config.into(),
            move |data, _: &_| write_input_data::<i8>(data),
            err_fn,
            None,
        )?,
        cpal::SampleFormat::I16 => device.build_input_stream(
            &config.into(),
            move |data, _: &_| write_input_data::<i16>(data),
            err_fn,
            None,
        )?,
        cpal::SampleFormat::I32 => device.build_input_stream(
            &config.into(),
            move |data, _: &_| write_input_data::<i32>(data),
            err_fn,
            None,
        )?,
        cpal::SampleFormat::F32 => device.build_input_stream(
            &config.into(),
            move |data, _: &_| write_input_data::<f32>(data),
            err_fn,
            None,
        )?,
        sample_format => {
            return Err(anyhow::Error::msg(format!(
                "Unsupported sample format '{sample_format}'"
            )));
        }
    };

    stream.play()?;

    // Let recording go for roughly three seconds.
    std::thread::sleep(std::time::Duration::from_secs(opt.duration));
    drop(stream);

    Ok(())
}

fn write_input_data<T>(input: &[T])
where
    T: Sample,
{
    println!("Received input data: {:?}", input.len());
    // for &sample in input.iter() {
    //     let sample: U = U::from_sample(sample);
    //     writer.write_sample(sample).ok();
    // }
}
