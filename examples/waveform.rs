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

use bevy_audio_widgets::wave_buffer::*;

const SAMPLE_SIZE: usize = 48000;
const F: f32 = 110.0;
const WINDOW: usize = (4.0 * (SAMPLE_SIZE as f32) / F) as usize;

#[derive(Resource)]
struct WaveForm(WaveReader<'static, SAMPLE_SIZE>);

fn main() -> Result<(), anyhow::Error> {
    // Create a static mutable WaveBuffer and split it into a writer and reader.
    let wave_buffer: &'static mut WaveBuffer<SAMPLE_SIZE> =
        Box::leak(Box::new(WaveBuffer::<SAMPLE_SIZE>::new()));

    let (mut writer, reader) = wave_buffer.split();

    // Callback closure for audio input stream. It will be called whenever new audio data is available.
    let write_input_data = {
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
    mut wave_reader: ResMut<WaveForm>,
) {
    gizmos.linestrip_2d(
        wave_reader
            .0
            .to_iterator(WINDOW)
            .enumerate()
            .map(|(i, sample)| {
                let t = i as f32 / WINDOW as f32;
                let x = (t - 0.5) * 600.0;
                let y = sample * 200.0;
                Vec2::new(x, y)
            }),
        WHITE,
    );
}
