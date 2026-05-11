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

struct WaveBuffer<const N: usize> {
    samples: [f32; N],
    index: usize,
}

impl<const N: usize> WaveBuffer<N> {
    fn new() -> Self {
        Self {
            samples: [0.0; N],
            index: 0,
        }
    }

    #[inline(always)]
    // Wraps around when the buffer is full.
    fn add_samples(&mut self, input: &[f32]) {
        for &sample in input.iter() {
            if self.index == 0 {
                println!(".");
            }
            self.samples[self.index] = sample;
            //  println!("sample: {:.2} at index {}", sample, self.index);
            self.index = (self.index + 1) % self.samples.len();
        }
    }
}

const SAMPLE_SIZE: usize = 1000;
#[derive(Resource)]
struct WaveForm<const N: usize>(Arc<Mutex<WaveBuffer<N>>>);

fn main() -> Result<(), anyhow::Error> {
    let wave_buffer = Arc::new(Mutex::new(WaveBuffer::<SAMPLE_SIZE>::new()));

    let wave_buffer_clone = wave_buffer.clone();
    let write_input_data = {
        move |input: &[f32]| {
            // println!("Received input data: {:?}", input.len());
            let mut wb = wave_buffer_clone.lock().unwrap();
            for &sample in input.iter() {
                wb.add_samples(&[sample]);
            }
        }
    };

    let (_opt, stream) = parse_input(write_input_data)?;
    stream.play()?;

    App::new()
        .add_plugins((
            DefaultPlugins,
            FpsOverlayPlugin {
                config: FpsOverlayConfig::default(),
            },
        ))
        .insert_resource(WaveForm::<SAMPLE_SIZE>(wave_buffer))
        .add_systems(Startup, setup)
        .add_systems(Update, (draw_waveform::<SAMPLE_SIZE>,))
        .run();
    Ok(())
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn draw_waveform<const N: usize>(
    mut gizmos: Gizmos,
    // _time: Res<Time>,
    wave_buffer: Res<WaveForm<N>>,
) {
    let wave_buffer = &*wave_buffer.0.lock().unwrap();

    gizmos.linestrip_2d(
        (0..N).map(|n| {
            let t = n as f32 / wave_buffer.samples.len() as f32;
            let x = (t - 0.5) * 600.0;
            let y = wave_buffer.samples[n] * 200.0;
            Vec2::new(x, y)
        }),
        WHITE,
    );
}
