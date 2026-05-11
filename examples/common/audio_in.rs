//! Audio input helper function.

use clap::Parser;
// use cpal::SampleFormat;
use cpal::StreamConfig;
use cpal::traits::{DeviceTrait, HostTrait /* StreamTrait */};
// use cpal::{FromSample, Sample};
// use std::sync::{Arc, Mutex};

#[derive(Parser, Debug)]
#[command(version, about = "CPAL record_wav example", long_about = None)]
pub struct Opt {
    /// The audio device to use.
    #[arg(short, long)]
    pub device: Option<String>,

    /// How long to record, in seconds
    #[arg(long, default_value_t = 1)]
    pub duration: u64,

    /// Channels
    #[arg(long, default_value_t = 1)]
    pub channels: u16,

    /// Sample rate
    #[arg(long, default_value_t = 48000)]
    pub sample_rate: u32,

    /// Buffer size in frames
    #[arg(long, default_value_t = 1024)]
    pub buffer_size: u32,

    /// Use the JACK host
    #[cfg(any(
        target_os = "linux",
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "netbsd"
    ))]
    #[arg(short, long)]
    #[allow(dead_code)]
    pub jack: bool,
}

#[allow(unused)]
pub fn parse_input(
    mut write_input_data: impl Fn(&[f32]) + Send + 'static,
) -> Result<(Opt, cpal::Stream), anyhow::Error> {
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
    let device = if let Some(device) = opt.device.clone() {
        let id = &device.parse().expect("failed to parse input device id");
        host.device_by_id(id)
    } else {
        host.default_input_device()
    }
    .expect("failed to find input device");

    println!("Input device: {}", device.id()?);

    let supported_configs = device.supported_input_configs()?;

    // Create a config according to user options with defaults:
    // - channels: 1
    // - sample_rate: 48_000
    // - buffer_size: 1024
    let config = StreamConfig {
        channels: opt.channels,
        sample_rate: opt.sample_rate,
        buffer_size: cpal::BufferSize::Fixed(opt.buffer_size),
    };

    println!("Input device config: {:?}", config);

    let err_fn = move |err| {
        eprintln!("an error occurred on stream: {err}");
    };

    let supported_txt = supported_configs
        .map(|config| {
            let sample_format = config.sample_format();
            let channels = config.channels();
            let min_sample_rate = config.min_sample_rate();
            let max_sample_rate = config.max_sample_rate();
            let buffer_size = config.buffer_size();
            format!(
                "Sample Format: {:?}, Channels: {}, Sample Rate: {}-{} Hz, Buffer Size: {:?}",
                sample_format, channels, min_sample_rate, max_sample_rate, buffer_size
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    let support_err = format!(
        "Failed to build input stream\nSupported configs:\n{}\nRequested config: {:?}\n",
        supported_txt, config
    );

    let stream = device
        .build_input_stream(
            &config,
            move |data: &[f32], info| {
                // println!("info: {:?}", info);
                write_input_data(data)
            },
            err_fn,
            None,
        )
        .expect(&support_err);

    println!("Input stream created");
    Ok((opt, stream))
}
