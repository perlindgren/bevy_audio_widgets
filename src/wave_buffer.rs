use log::{Level, debug, log_enabled, trace};
pub struct WaveBuffer {
    pub rate: f32,         // sample rate
    pub samples: Vec<f32>, // buffer size
    pub index: usize,      // next index to write in buffer
}

impl WaveBuffer {
    pub fn new(rate: f32, len: usize) -> Self {
        Self {
            rate,
            samples: vec![0.0; len],
            index: 0, // we start at index 0.
        }
    }

    #[inline(always)]
    // Wraps around when the buffer is full.
    pub fn add_samples(&mut self, input: &[f32]) {
        // index is used for the raw buffer accesses, so modulo self.samples.len().
        let mut index = self.index % self.samples.len();

        for &sample in input.iter() {
            self.samples[index] = sample;
            index = (index + 1) % self.samples.len();

            if log_enabled!(Level::Trace) {
                if index == 0 {
                    trace!("Input buffer wrapping around...");
                }
                trace!("sample: {:.2} at index {}", sample, index);
            }
        }
        self.index += input.len();
    }

    // Returns an iterator over the n most recent samples in the buffer, phase aligned to the requested frequency
    pub fn to_iterator(&self, freq: f32) -> impl Iterator<Item = f32> + '_ {
        let frame_size = self.rate / freq;
        let index = self.index;
        let offset = (self.index as f32 % frame_size) as usize;
        if log_enabled!(Level::Debug) {
            debug!(
                "Creating iterator with frame size {}, index {}, offset {}",
                frame_size, index, offset
            );
        }

        // iterator i1, containing the most recent samples
        let i1 = WaveBufferIter {
            buffer: self,
            index,
            nr_items: offset,
        };

        // iterator i2, containing the older samples from previous frame
        let i2 = WaveBufferIter {
            buffer: self,
            index: self.samples.len() + self.index - offset,
            nr_items: frame_size as usize - offset,
        };

        // returned iterator is one frame of audio data
        i2.chain(i1)
    }
}

struct WaveBufferIter<'a> {
    buffer: &'a WaveBuffer,
    index: usize,
    nr_items: usize,
}

impl std::iter::Iterator for WaveBufferIter<'_> {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.nr_items == 0 {
            return None;
        }
        let len = self.buffer.samples.len();
        // index points no next, so decrement first
        // wrapping is handled by modulo
        self.index = (len + self.index - 1) % len;
        let sample = self.buffer.samples[self.index];
        self.nr_items -= 1;
        Some(sample)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wave_buffer() {
        let mut buffer = WaveBuffer::new(15.0, 20);
        for i in 0..25 {
            buffer.add_samples(&[i as f32]);
        }
        println!("Buffer index: {:?}", buffer.index);
        assert!(buffer.index == 25);

        let iter = buffer.to_iterator(1.0);
        for s in iter {
            println!("Sample: {}", s);
        }

        println!("---");
        for i in 25..30 {
            buffer.add_samples(&[i as f32]);
        }
        assert!(buffer.index == 30);

        let iter = buffer.to_iterator(1.0);
        for s in iter {
            println!("Sample: {}", s);
        }

        println!("---");
        for i in 30..35 {
            buffer.add_samples(&[i as f32]);
        }
        assert!(buffer.index == 35);
        let iter = buffer.to_iterator(1.0);
        for s in iter {
            println!("Sample: {}", s);
        }
    }
}
