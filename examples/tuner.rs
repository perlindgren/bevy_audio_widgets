use bevy::{
    color::palettes::css::*,
    dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin},
    prelude::*,
};
use cpal::traits::StreamTrait;

mod common;
use common::audio_in::{open_input_stream, parse_input};
use std::sync::{Arc, Mutex};

use bevy_audio_widgets::wave_buffer::*;

#[derive(Resource)]
struct WaveForm(Arc<Mutex<WaveBuffer>>);

const SAMPLE_SIZE: usize = 48000;

fn main() -> Result<(), anyhow::Error> {
    env_logger::init();

    let opt = parse_input();
    let buf_size = SAMPLE_SIZE;

    // E2, A3, D3, G3, B3, E4, the standard tuning frequencies for a guitar
    let tuning_frequencies = [82.41, 110.0, 146.83, 196.0, 246.94, 329.63];

    let wave_buffer = Arc::new(Mutex::new(WaveBuffer::new(
        opt.sample_rate as f32,
        buf_size,
        opt.nr_periods,
        &tuning_frequencies,
    )));

    let wave_buffer_clone = wave_buffer.clone();

    // Callback closure for audio input stream. It will be called whenever new audio data is available.
    let write_input_data = {
        move |input: &[f32]| {
            trace!("Received input data: {:?}", input.len());
            let mut wb = wave_buffer_clone.lock().unwrap();
            wb.add_samples(input);
        }
    };

    let stream = open_input_stream(&opt, write_input_data)?;

    println!(
        "Starting audio input stream... sample rate: {}, channels: {}",
        opt.sample_rate, opt.channels
    );

    stream.play()?;

    App::new()
        .add_plugins((
            DefaultPlugins,
            // DefaultPlugins.build().disable::<bevy::log::LogPlugin>(),
            FpsOverlayPlugin {
                config: FpsOverlayConfig::default(),
            },
        ))
        .insert_resource(WaveForm(wave_buffer))
        .add_systems(Startup, setup)
        .add_systems(Update, (draw_waveform,))
        .run();
    Ok(())
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn draw_waveform(mut gizmos: Gizmos, wave_buffer: Res<WaveForm>) {
    let wave_buffer = &*wave_buffer.0.lock().unwrap();

    let mut draw = |freq: f32, y: f32, color: Color| {
        let (frame_size, frame_it) = wave_buffer.to_iterator(4.0, freq);
        let it = frame_it.enumerate().map(|(n, sample)| {
            let t = n as f32 / frame_size;
            let x = (t - 0.5) * 600.0;
            let y = sample * 1000.0 + y;
            Vec2::new(x, y)
        });
        gizmos.linestrip_2d(it, color);
    };

    // compute match
    for tf in wave_buffer.frame_buffers.iter() {
        let (frame_size, it) = wave_buffer.to_iterator(1.0, tf.tuning_frequency);
        print!(
            "tf {}: it {}, fs {}",
            tf.tuning_frequency,
            it.count(),
            frame_size
        );
    }
    println!();

    draw(82.41, -200.0, GREEN.into()); // E2

    // let freq = 82.41 / 4.0; // E2
    // let it = wave_buffer
    //     .to_iterator(freq)
    //     .enumerate()
    //     .map(|(n, sample)| {
    //         let t = n as f32 / (len as f32 / freq);
    //         let x = (t - 0.5) * 600.0;
    //         let y = sample * 100.0 - 200.0;
    //         Vec2::new(x, y)
    //     });
    // gizmos.linestrip_2d(it, GREEN);

    // let freq = 110.0; // A3
    // let it = wave_buffer
    //     .to_iterator(freq)
    //     .enumerate()
    //     .map(|(n, sample)| {
    //         let t = n as f32 / (len as f32 / freq);
    //         let x = (t - 0.5) * 600.0;
    //         let y = sample * 100.0;
    //         Vec2::new(x, y)
    //     });
    // gizmos.linestrip_2d(it, GREEN);

    // let freq = 146.83; // D3
    // let it = wave_buffer
    //     .to_iterator(freq)
    //     .enumerate()
    //     .map(|(n, sample)| {
    //         let t = n as f32 / (len as f32 / freq);
    //         let x = (t - 0.5) * 600.0;
    //         let y = sample * 100.0 + 200.0;
    //         Vec2::new(x, y)
    //     });
    // gizmos.linestrip_2d(it, LIGHT_YELLOW);

    // let freq = 196.0; // G3
    // let it = wave_buffer
    //     .to_iterator(freq)
    //     .enumerate()
    //     .map(|(n, sample)| {
    //         let t = n as f32 / (len as f32 / freq);
    //         let x = (t - 0.5) * 600.0;
    //         let y = sample * 100.0 + 400.0;
    //         Vec2::new(x, y)
    //     });
    // gizmos.linestrip_2d(it, LIGHT_BLUE);
}
