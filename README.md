# Bevy Audio Widgets

The intention is to provide a set of re-usable widgets for audio visualizations including:

- Waveform views.
- Frequency spectrum analysis, e.g., spectrogram.
- Peak and RMS metering.

Example applications.

- Tuner for stringed instruments.

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

It is based on an oscilloscope view of the signal, where an in tune signal is steady while an out of tune signal wanders either to the right or to the left. As we will later see, this approach is superior to traditional digital tuners as it provides immediate (zero latency) feedback, with precision beyond the resonance stability of any stringed instrument. Furthermore it allows for tuning against both open strings, fretted notes and (pinch) harmonics without any loss of precision. In fact the taken approach reveals shortcomings of string material, saddles, tuners, and effects of magnetic pull induced by the microphones. Thus besides of tuning, it provides a tool for precise adjustments of intonation, microphone height and other parameters of the stringed instrument.

#### Implementation details

Assuming we want to tune for the E2 (the lowest string of a 6-stringed guitar in standard tuning). The frequency of E2 is 82.41Hz with a corresponding period of 1/82.41. Assuming a sample rate of 48000 Hz, the frame length is 582.45358573 (notice we do not round at this point, which will be of importance later).

The wave buffer (backing store) is handled akin to a single producer multiple (SPMC) consumer pattern. The consumer provides dual iterators ($i1$, $i2$) over a $period$, where $i1$ and and $i2$ where $i1$ provides the samples from frame $offset$ to start of frame (most recent first), and $i2$ provides the remaining (older) samples of the previous frame.

Assume $n$ to be the (monotonic growing) index of the last sample written by the producer, then:

$$offset = n\%frame$$

This calculation is performed on the floating point representation of the frame length to increase the precision. The view composes the iterator outputs where the last (oldest) sample of $i2$ is followed by the first (most recent sample of $i1$), allowing a continuos line to be formed from the sample points.

For the implementation, we adopt the Rust iterator pattern in a zero copy fashion.

Figures below depict edge cases:

- a), in this case $n$ is at the last index of the buffer. The $offset$ is calculated from the last frame.

- b), in this case $n$ has surpassed the length of the frame. The $offset$ is (still) calculated from the last frame. Notice however, that the generated iterators refer to indices modulus the buffer length (thus all reads are ensured to be within bounds of the shared backing store).

![wavebuf](./figs/wavebuf2.drawio.svg)

As an aside, we observe that the waveform is discontinuous across $n$, where new samples overwrite old (this, even in case that the input samples forms a continuous waveform). The $view$ however, always present a phase aligned representation in case the frame length matches a period (or harmonic) of the input signal.

The monotonically increasing counter $n$ is stored as a `usize`. With a sample rate of 48kHz, we will run into an overflow after $(2^64-1)/48000/60/60/24/356=12186300$ years, thus an Ariane 5 flight 501 failure is unlikely.

### Performance and further improvements

Running the tuner application with 6 matching frames concurrently (including rendering) seems to consume less than one percent of available CPU resources on tested arch Linux x86. On tested MacOS M4 (aarch64) Activity monitor reports 50% of a single core. Thus performance is sufficiently good for practical use (even in debug build).

However, the current implementation can be improved, e.g., by:

- Removing the mutex lock requirement, by storing data as atomic (`f32`), thus overwrites can be safely implemented. If desired atomic `usize` can hold the write index and read index/indexes cross the current write index can be checked indicating buffer overruns.

- Since all accesses to underlying storage are done through vector indexes under modulus arithmetics, no range checking is needed. This can be achieved through _unsafe_ `get_mut` and `get` operations for the producer/consumer(s) respectively.

Why not instead you const generics arrays. Well, this is a design decision based on flexibility, we do not want to require the buffer size to be determined at compile time, thus a no go for const generics in this case.

---

## Running

To run the tuner application on a simulated sine wave:

```shell
RUST_LOG=bevy=info,tuner=error cargo run --example tuner -- --sine-wave-freq 110.0 --buffer-size 256
```

Under a jack enabled Linux environment, the `--jack` option should be added for low latency audio capture.

Logging is supported by the `log` shim. For the examples `env_logger` is used, as in the example above (allowing different logging details per crate, application, module etc.)

Bevy by itself implements a logging framework by default. Bevy will report an error as the tuner example has already created a logging backend (in order to provide logging before Bevy is instantiated/called). This error can be safely ignored. If for some reason you do not want Bevy logging at all, the `LogPlugin` can be disabled (with the effect that no error will be reported by Bevy).

### Bugs

It seems that when running the tuner on a generated sine wave, over time (several minutes) the waveform view suffers noise. It is unclear if the noise stems from the generated sine wave, or if it stems from the buffer handling. The problem is under investigation.

### TODOS

The crate (library and examples) are under early development. Some planned features include:

- ASIO support for Windows

- More widgets and examples
  - Peak and RMS metering
  - Auto gain (compressor like effect) for improving visual feedback
  - Spectrum visualizations (instance, and over time)

## Contributions Welcome

Contributions are welcome. Feel free to raise issues, and/or submit PRs according to the below License. Vibe coded contributions will be discarded, while the use of AI assisted tooling is allowed unless breaking with below License.

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
