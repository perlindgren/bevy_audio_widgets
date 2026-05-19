use bevy::{
    color::palettes::css::*,
    dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin},
    // math::Isometry2d,
    prelude::*,
};
use cpal::traits::StreamTrait;
// use std::f32::consts::{FRAC_PI_2, PI, TAU};
mod common;
use common::audio_in::parse_input;
use std::sync::{Arc, Mutex};

use bevy_audio_widgets::wave_buffer::*;

#[derive(Resource)]
struct WaveForm(Arc<Mutex<WaveBuffer>>);

const SAMPLE_SIZE: usize = 48000;

fn main() -> Result<(), anyhow::Error> {
    let opt = parse_input();
    let buf_size = SAMPLE_SIZE;

    let wave_buffer = Arc::new(Mutex::new(WaveBuffer::new(
        opt.sample_rate as f32,
        buf_size,
    )));

    let wave_buffer_clone = wave_buffer.clone();

    // Callback closure for audio input stream. It will be called whenever new audio data is available.
    let write_input_data = {
        move |input: &[f32]| {
            // println!("Received input data: {:?}", input.len());
            let mut wb = wave_buffer_clone.lock().unwrap();
            wb.add_samples(input);
        }
    };

    let stream = common::audio_in::open_input_stream(&opt, write_input_data)?;

    println!(
        "Starting audio input stream... sample rate: {}, channels: {}",
        opt.sample_rate, opt.channels
    );

    stream.play()?;

    App::new()
        .add_plugins((
            DefaultPlugins,
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

    let len = wave_buffer.samples.len();
    // gizmos.linestrip_2d(
    //     (0..len).map(|n| {
    //         let t = n as f32 / wave_buffer.samples.len() as f32;
    //         let x = (t - 0.5) * 600.0;
    //         let y = wave_buffer.samples[n] * 50.0;
    //         Vec2::new(x, y - 500.0)
    //     }),
    //     WHITE,
    // );

    let freq = 82.41; // E2
    let it = wave_buffer
        .to_iterator(freq)
        .enumerate()
        .map(|(n, sample)| {
            let t = n as f32 / (len as f32 / freq);
            let x = (t - 0.5) * 600.0;
            let y = sample * 100.0 - 200.0;
            Vec2::new(x, y)
        });
    gizmos.linestrip_2d(it, GREEN);

    let freq = 110.0; // A3
    let it = wave_buffer
        .to_iterator(freq)
        .enumerate()
        .map(|(n, sample)| {
            let t = n as f32 / (len as f32 / freq);
            let x = (t - 0.5) * 600.0;
            let y = sample * 100.0;
            Vec2::new(x, y)
        });
    gizmos.linestrip_2d(it, GREEN);

    let freq = 146.83; // D3
    let it = wave_buffer
        .to_iterator(freq)
        .enumerate()
        .map(|(n, sample)| {
            let t = n as f32 / (len as f32 / freq);
            let x = (t - 0.5) * 600.0;
            let y = sample * 100.0 + 200.0;
            Vec2::new(x, y)
        });
    gizmos.linestrip_2d(it, LIGHT_YELLOW);

    let freq = 196.0; // G3
    let it = wave_buffer
        .to_iterator(freq)
        .enumerate()
        .map(|(n, sample)| {
            let t = n as f32 / (len as f32 / freq);
            let x = (t - 0.5) * 600.0;
            let y = sample * 100.0 + 400.0;
            Vec2::new(x, y)
        });
    gizmos.linestrip_2d(it, LIGHT_BLUE);
}
