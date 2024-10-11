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
- A recent OpenJDK installation (Java 21+), located at `JAVA_HOME`.
- A recent GraalVM installation (Java 21+), located at `GRAALVM_HOME`.
- Maven (for building the Java part of the demo).
- A recent MoonBit toolchain. You can install it from [moonbitlang.com](https://www.moonbitlang.com/download).

After that, you can ensure that both demos are working properly by running:

```bash
cargo run
```

To run the benchmarks, just run:

```bash
cargo bench
```

On my MacBook Pro M1 Pro, this gives something like the following:

```console
[..]
Timer precision: 41 ns
fft                fastest       │ slowest       │ median        │ mean          │ samples │ iters
╰─ bench_fft_demo                │               │               │               │         │
   ├─ GraalVM      54.53 ms      │ 61.28 ms      │ 56.14 ms      │ 56.39 ms      │ 100     │ 100
   ├─ MoonBit      16.62 ms      │ 17.65 ms      │ 16.92 ms      │ 16.96 ms      │ 100     │ 100
   ╰─ OpenJDK      188.6 ms      │ 219.6 ms      │ 203.5 ms      │ 203.9 ms      │ 100     │ 100
```
