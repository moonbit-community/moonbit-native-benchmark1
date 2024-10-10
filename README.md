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
...
Timer precision: 41 ns
fft                fastest       │ slowest       │ median        │ mean          │ samples │ iters
╰─ bench_fft_demo                │               │               │               │         │
   ├─ Java         189.6 ms      │ 215.4 ms      │ 203.3 ms      │ 203.6 ms      │ 100     │ 100
   ╰─ MoonBit      87.74 ms      │ 93.86 ms      │ 88.63 ms      │ 88.8 ms       │ 100     │ 100
```
