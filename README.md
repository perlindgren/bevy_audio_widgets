# Bevy Audio Widgets

The intention is to provide a set of re-usable widgets for audio visualizations including:

- Wave form views.
- Frequency spectrum analysis.
- Spectrogram.

## Dependencies

For the examples we depend on various crates within the Rust ecosystem.

- [clap](https://crates.io/crates/clap) with `features = "derive".

## Buffer handling

Buffer handling is at heart of all audio applications. For the purpose of audio visualization we adopt a pre-allocated vector, where data is shared between the capture and the widgets, where widgets are presented with read only view underlying data.

![wavebuf](./figs/wavebuf.drawio.svg)

The figure above illustrates the capture and view.

- capture, enqueues incoming audio into the (circular) buffer. This is typically done by the "audio" thread.
- view, processes the buffered audio.

---

### Tuner application

As an example we develop a (guitar) tuner application.

It is based on an oscilloscope view of the signal, where an in tune signal is steady while an out of tune signal wanders either to the right or to the left. As we will later see, this approach is superior to traditional digital tuners as it provides immediate (zero latency) feedback, with precision beyond the resonance stability of any stringed instrument. Furthermore it allows for tuning against both open and (pinch) harmonics any loss of precision. In fact the taken approach reveals shortcomings of string material, saddles, tuners, and effects of magnetic pull induced by the microphones. Thus besides of tuning, it provides a tool for precise adjustments of intonation and microphone height.

#### Implementation details

Assuming we want to tune for the E2 (the lowest string of a 6-stringed guitar in standard tuning). The frequency of E2 is 82.41Hz with a corresponding period of 1/82.41. Assuming a sample rate of 48000 Hz, the frame length is 582.45358573 (notice we do not round at this point, which will be of importance later).

The wave buffer is spitted akin to a single producer multiple (SPMC) consumer pattern. The consumer provides dual iterators ($i1$, $i2$) over a $period$, where $i1$ and and $i2$ where $i1$ provides the samples from frame $offset$ to start of frame (most recent first), and $i2$ provides the remaining (older) samples of the previous frame (wrapping around if needed). Assume $n$ to be the last sample written by the producer, then:

$$offset = n\%frame$$

This calculation is performed on the floating point representation of the frame length to increase the precision. The view composes the iterator outputs where the last (oldest) sample of $i2$ is followed by the first (most recent sample of $i2$), allowing a continuos line to be formed from the sample points.

For the implementation, we adopt the Rust iterator pattern in a zero copy fashion.

## License

MIT License

Copyright (c) 2026 Per Lindgren

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
