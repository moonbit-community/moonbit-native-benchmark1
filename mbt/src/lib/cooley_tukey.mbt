/// The function implementing the Cooley–Tukey algorithm for Fast Fourier Transform (FFT), as
/// described in <https://cp-algorithms.com/algebra/fft.html#implementation>.
///
/// The input signal is modified in-place.
pub fn fft(arr : Array[Complex]) -> Unit {
  fn _fft(arr : Array[Complex]) {
    let n = arr.length()
    if n == 1 {
      return
    }
    let a0 = Array::new(capacity=n / 2)
    let a1 = Array::new(capacity=n / 2)
    for i in 0..<(n / 2) {
      a0.push(arr[2 * i])
      a1.push(arr[2 * i + 1])
    }
    _fft(a0)
    _fft(a1)
    let ang = -2.0 * @math.pi / Double::from_int(n)
    let mut w = { real: 1.0, imag: 0.0 }
    let wn = { real: @math.cos(ang), imag: @math.sin(ang) }
    for i in 0..<(n / 2) {
      let p = a0[i]
      let q = w.mul(a1[i])
      arr[i] = p.add(q)
      arr[i + n / 2] = p.sub(q)
      w = w.mul(wn)
    }
  }

  _fft(arr)
  let factor = 1.0 / Double::from_int(arr.length()).sqrt()
  for i in 0..<arr.length() {
    arr[i] = arr[i].mul_scalar(factor)
  }
}
