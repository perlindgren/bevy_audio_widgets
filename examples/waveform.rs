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

struct WaveBuffer {
    len: usize,
    samples: Vec<f32>,
    index: usize,
}

impl WaveBuffer {
    fn new(len: usize) -> Self {
        Self {
            len,
            samples: vec![0.0; len],
            index: 0,
        }
    }

    #[inline(always)]
    // Wraps around when the buffer is full.
    fn add_samples(&mut self, input: &[f32]) {
        for &sample in input.iter() {
            // if self.index == 0 {
            //     println!(".");
            // }
            self.samples[self.index] = sample;
            //  println!("sample: {:.2} at index {}", sample, self.index);
            self.index = (self.index + 1) % self.samples.len();
        }
    }
}

#[derive(Resource)]
struct WaveForm(Arc<Mutex<WaveBuffer>>);

fn main() -> Result<(), anyhow::Error> {
    let SAMPLE_SIZE: usize = (2.0 * 48000.0 / 110.0) as usize;

    let wave_buffer = Arc::new(Mutex::new(WaveBuffer::new(SAMPLE_SIZE)));

    let wave_buffer_clone = wave_buffer.clone();

    // Callback closure for audio input stream. It will be called whenever new audio data is available.
    let write_input_data = {
        move |input: &[f32]| {
            // println!("Received input data: {:?}", input.len());
            let mut wb = wave_buffer_clone.lock().unwrap();
            wb.add_samples(input);
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
        .insert_resource(WaveForm(wave_buffer))
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
