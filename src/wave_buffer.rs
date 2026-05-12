use std::iter::Iterator;

#[derive(Debug)]
pub struct WaveBuffer {
    samples: Vec<f32>, // this will be a fixed size buffer
    in_index: usize,
}

impl WaveBuffer {
    pub fn new(len: usize) -> Self {
        Self {
            samples: vec![0.0; len],
            in_index: 0,
        }
    }

    fn as_wave_writer(&mut self) -> WaveWriter<'_> {
        WaveWriter { buffer: self }
    }

    fn as_wave_reader(&self) -> WaveReader<'_> {
        WaveReader { buffer: self }
    }

    pub fn split(&mut self) -> (WaveWriter<'_>, WaveReader<'_>) {
        let my_self: *mut WaveBuffer = self;
        let ww = unsafe { &mut *my_self }.as_wave_writer();
        let wr = unsafe { &*my_self }.as_wave_reader();
        (ww, wr)
    }
}

pub struct WaveWriter<'a> {
    buffer: &'a mut WaveBuffer,
}

impl<'a> WaveWriter<'a> {
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

pub struct WaveReader<'a> {
    buffer: &'a WaveBuffer,
}

impl<'a> WaveReader<'a> {
    #[inline(always)]
    // Wraps around when the buffer is full.
    pub fn to_iterator(&'a self, nr_elements: usize) -> WaveReaderIter<'a> {
        WaveReaderIter {
            reader: self,
            index: self.buffer.in_index, // start reading from the current write index
            nr_elements,
        }
    }
}

pub struct WaveReaderIter<'a> {
    reader: &'a WaveReader<'a>,
    index: usize,
    nr_elements: usize,
}

impl<'a> Iterator for WaveReaderIter<'a> {
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
        let mut buffer = WaveBuffer::new(5);
        let (mut writer, reader) = buffer.split();

        writer.add_samples(&[1.0, 2.0, 3.0]); // last added to far right

        let collected: Vec<f32> = reader.to_iterator(3).collect();
        println!("Collected samples: {:?}", collected);
        assert_eq!(collected, vec![3.0, 2.0, 1.0]);
    }
}
