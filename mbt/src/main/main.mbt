typealias Complex = @lib.Complex

fn main {
  let signals = generate_inputs(16384)
  @lib.fft(signals)
  for signal in signals {
    println("\{round(signal.real)},\{round(signal.imag)}")
  }
}

fn round(n : Double) -> Double {
  // precision = 2
  (n * 100.0).round() / 100.0
}

fn generate_inputs(len : Int) -> Array[Complex] {
  let res : Array[Complex] = []
  for i in 0..<len {
    let theta = i.to_double() / len.to_double() * @math.pi
    let re = 1.0 * @math.cos(10.0 * theta) + 0.5 * @math.cos(25.0 * theta)
    let im = 1.0 * @math.sin(10.0 * theta) + 0.5 * @math.sin(25.0 * theta)
    res.push({ real: round(re), imag: round(im) })
  }
  res
}
