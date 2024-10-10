# native-showcase-fft

This demo/benchmark showcases the performance of MoonBit's native backend
with the Fast Fourier Transform,
which is useful in many applications such as audio processing, image processing, and data compression.

We use Java 21+ as the baseline for comparison,
and the FFT implementations in this demo (both Java and MoonBit native
versions) are based on the Cooley-Tukey algorithm,
implemented naively without further optimizations.

## Running the demo

Please ensure that you have the following prerequisites installed:

- A recent Rust toolchain. You can install it from [rustup.rs](https://rustup.rs).
- A recent (21+) Java Development Kit (we recommend OpenJDK).
- Maven (for building the Java part of the demo).
- A recent MoonBit toolchain. You can install it from [moonbitlang.com](https://www.moonbitlang.com/download).

After that, you can run the demo by simply executing:

```bash
cargo run
```
