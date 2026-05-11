use bevy::{color::palettes::css::*, math::Isometry2d, prelude::*};
use cpal::traits::StreamTrait;
use std::f32::consts::{FRAC_PI_2, PI, TAU};
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
            self.samples[self.index] = sample;
            //  println!("sample: {:.2} at index {}", sample, self.index);
            self.index = (self.index + 1) % self.samples.len();
        }
    }
}

impl Curve<Vec2> for WaveBuffer<1024> {
    // Required methods
    fn domain(&self) -> Interval {
        Interval::EVERYWHERE
    }

    fn sample_unchecked(&self, t: f32) -> Vec2 {
        let index = ((t.fract() + 1.0) % 1.0 * self.samples.len() as f32) as usize;
        let y = self.samples[index] * 200.0;
        // println!("Sampling at t={:.2}: index={}, y={:.2}", t, index, y);
        Vec2::new(t, y)
    }
}

#[derive(Resource)]
struct WaveForm(Arc<Mutex<WaveBuffer<1024>>>);

fn main() -> Result<(), anyhow::Error> {
    let wave_buffer = Arc::new(Mutex::new(WaveBuffer::<1024>::new()));

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
        .add_plugins(DefaultPlugins)
        .insert_resource(WaveForm(wave_buffer))
        .add_systems(Startup, setup)
        .add_systems(Update, (draw_waveform,))
        .run();
    Ok(())
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn draw_waveform(mut gizmos: Gizmos, time: Res<Time>, wave_buffer: Res<WaveForm>) {
    let domain = Interval::EVERYWHERE;
    let curve = FunctionCurve::new(domain, |t| Vec2::new(t, ops::sin(t / 25.0) * 100.0));
    let resolution = ((ops::sin(time.elapsed_secs()) + 1.0) * 50.0) as usize;

    let wave_buffer = &*wave_buffer.0.lock().unwrap();

    gizmos.curve_2d(
        &*wave_buffer,
        (0..=wave_buffer.samples.len())
            .map(|n| n as f32 / wave_buffer.samples.len() as f32)
            .map(|t| (t - 0.5) * 600.0),
        WHITE,
    );
}
