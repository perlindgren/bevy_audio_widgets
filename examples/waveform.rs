use bevy::{
    color::palettes::css::*,
    dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin},
    // math::Isometry2d,
    prelude::*,
    render::render_resource::encase::private::Reader,
};
use cpal::traits::StreamTrait;
// use std::f32::consts::{FRAC_PI_2, PI, TAU};
mod common;
use common::audio_in::parse_input;
use std::sync::{Arc, Mutex};

use bevy_audio_widgets::wave_buffer::*;

const SAMPLE_SIZE: usize = 48000;

#[derive(Resource)]
struct WaveForm(WaveReader<'static, SAMPLE_SIZE>);

fn main() -> Result<(), anyhow::Error> {
    let mut wave_buffer = WaveBuffer::<SAMPLE_SIZE>::new();

    let (mut writer, reader) = wave_buffer.split();

    // Callback closure for audio input stream. It will be called whenever new audio data is available.
    let mut write_input_data = {
        move |input: &[f32]| {
            // println!("Received input data: {:?}", input.len());
            writer.add_samples(input);
        }
    };

    let (_opt, stream) = parse_input(write_input_data)?;

    println!(
        "Starting audio input stream... sample rate: {}, channels: {}",
        _opt.sample_rate, _opt.channels
    );

    stream.play()?;

    App::new()
        .add_plugins((
            DefaultPlugins,
            FpsOverlayPlugin {
                config: FpsOverlayConfig::default(),
            },
        ))
        .insert_resource(WaveForm(reader))
        .add_systems(Startup, setup)
        .add_systems(Update, (draw_waveform,))
        .run();
    Ok(())
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn draw_waveform(
    mut gizmos: Gizmos,
    // _time: Res<Time>,
    wave_buffer: Res<WaveForm>,
) {
    let wave_buffer = &*wave_buffer.0.lock().unwrap();

    gizmos.linestrip_2d(
        (0..wave_buffer.len).map(|n| {
            let t = n as f32 / wave_buffer.samples.len() as f32;
            let x = (t - 0.5) * 600.0;
            let y = wave_buffer.samples[n] * 200.0;
            Vec2::new(x, y)
        }),
        WHITE,
    );
}
