use std::iter;

struct WaveBuffer {
    rate: f32,         // sample rate
    samples: Vec<f32>, // buffer size
    index: usize,      // next index to write in buffer
}

impl WaveBuffer {
    fn new(rate: f32, len: usize) -> Self {
        Self {
            rate,
            samples: vec![0.0; len],
            index: 0, // we start at index 0.
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
            self.index = (self.index + 1) % self.samples.len();
            if self.index == 0 {
                println!("Buffer full, wrapping around...");
            }

            //  println!("sample: {:.2} at index {}", sample, self.index);
        }
    }

    //
    fn to_iterator(&self, freq: f32) -> impl Iterator<Item = f32> + '_ {
        let len = self.samples.len();
        let frame_size = self.rate / freq;
        let index = self.index;
        let offset = (self.index as f32 % frame_size) as usize;
        println!(
            "self.index {}, index: {}, frame_size: {}",
            self.index, index, frame_size
        );

        let i1 = WaveBufferIter {
            buffer: self,
            index,
            nr_items: offset,
        };

        // i1

        let i2 = WaveBufferIter {
            buffer: self,
            index: (self.samples.len() + self.index - offset) % self.samples.len(), // start at the beginning of the frame
            nr_items: frame_size as usize - offset,
        };

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
        self.index = (len + self.index - 1) % len; // index points no next, so decrement first 
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
        assert!(buffer.index == 5);

        let iter = buffer.to_iterator(1.0);
        for s in iter {
            println!("Sample: {}", s);
        }

        println!("---");
        for i in 25..30 {
            buffer.add_samples(&[i as f32]);
        }
        assert!(buffer.index == 10);

        let iter = buffer.to_iterator(1.0);
        for s in iter {
            println!("Sample: {}", s);
        }

        println!("---");
        for i in 30..35 {
            buffer.add_samples(&[i as f32]);
        }
        assert!(buffer.index == 15);
        let iter = buffer.to_iterator(1.0);
        for s in iter {
            println!("Sample: {}", s);
        }
    }
}
