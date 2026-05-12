use std::iter::Iterator;

#[derive(Debug)]
pub struct WaveBuffer<const N: usize> {
    samples: [f32; N], // this will be a fixed size buffer
    in_index: usize,
}

impl<const N: usize> WaveBuffer<N> {
    #[allow(clippy::new_without_default)]
    pub const fn new() -> Self {
        Self {
            samples: [0.0; N], // initialize with zeros, fixed size
            in_index: 0,
        }
    }

    const fn as_wave_writer(&mut self) -> WaveWriter<'_, N> {
        WaveWriter { buffer: self }
    }

    const fn as_wave_reader(&self) -> WaveReader<'_, N> {
        WaveReader { buffer: self }
    }

    pub const fn split(&mut self) -> (WaveWriter<'_, N>, WaveReader<'_, N>) {
        let my_self: *mut WaveBuffer<N> = self;
        let ww = unsafe { &mut *my_self }.as_wave_writer();
        let wr = unsafe { &*my_self }.as_wave_reader();
        (ww, wr)
    }
}

pub struct WaveWriter<'a, const N: usize> {
    buffer: &'a mut WaveBuffer<N>,
}

impl<'a, const N: usize> WaveWriter<'a, N> {
    #[inline(always)]
    // Wraps around when the buffer is full.
    pub fn add_samples(&mut self, input: &[f32]) {
        for &sample in input.iter() {
            // if self.index == 0 {
            //     println!(".");
            // }
            self.buffer.samples[self.buffer.in_index] = sample;
            //  println!("sample: {:.2} at index {}", sample, self.index);
            self.buffer.in_index = (self.buffer.in_index + 1) % self.buffer.samples.len();
        }
    }
}

pub struct WaveReader<'a, const N: usize> {
    buffer: &'a WaveBuffer<N>,
}

impl<'a, const N: usize> WaveReader<'a, N> {
    #[inline(always)]
    // Wraps around when the buffer is full.
    pub fn to_iterator(&'a self, nr_elements: usize) -> WaveReaderIter<'a, N> {
        WaveReaderIter {
            reader: self,
            index: self.buffer.in_index, // start reading from the current write index
            nr_elements,
        }
    }
}

pub struct WaveReaderIter<'a, const N: usize> {
    reader: &'a WaveReader<'a, N>,
    index: usize,
    nr_elements: usize,
}

impl<'a, const N: usize> Iterator for WaveReaderIter<'a, N> {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.nr_elements == 0 {
            None
        } else {
            self.index = (self.index + self.reader.buffer.samples.len() - 1)
                % self.reader.buffer.samples.len();
            let sample = self.reader.buffer.samples[self.index];

            self.nr_elements -= 1;
            Some(sample)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wave_buffer() {
        let mut buffer = WaveBuffer::<5>::new();
        let (mut writer, reader) = buffer.split();

        writer.add_samples(&[1.0, 2.0, 3.0]); // last added to far right

        let collected: Vec<f32> = reader.to_iterator(3).collect();
        println!("Collected samples: {:?}", collected);
        assert_eq!(collected, vec![3.0, 2.0, 1.0]);
    }
}
