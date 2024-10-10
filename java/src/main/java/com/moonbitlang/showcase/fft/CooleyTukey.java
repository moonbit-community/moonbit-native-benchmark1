package com.moonbitlang.showcase.fft;

/**
 * The utility class implementing the Cooley–Tukey algorithm for Fast Fourier Transform (FFT), as
 * described in <href>https://cp-algorithms.com/algebra/fft.html#implementation</href>.
 */
public class CooleyTukey {
  /**
   * Performs the Fast Fourier Transform (FFT) algorithm by modifying the input signal in-place.
   *
   * @param arr the input signal
   */
  public static void fft(Complex[] arr) {
    fftImpl(arr);
    final var factor = 1.0 / Math.sqrt(arr.length);
    for (var i = 0; i < arr.length; i++) {
      arr[i] = arr[i].mul(factor);
    }
  }

  private static void fftImpl(Complex[] arr) {
    final var n = arr.length;
    if (n == 1) {
      return;
    }

    var a0 = new Complex[n / 2];
    var a1 = new Complex[n / 2];
    for (var i = 0; i < n / 2; i++) {
      a0[i] = arr[2 * i];
      a1[i] = arr[2 * i + 1];
    }

    fftImpl(a0);
    fftImpl(a1);

    final var ang = -2 * Math.PI / n;
    var w = new Complex(1, 0);
    final var wn = new Complex(Math.cos(ang), Math.sin(ang));
    for (int i = 0; i < n / 2; i++) {
      final var p = a0[i];
      final var q = w.mul(a1[i]);
      arr[i] = p.add(q);
      arr[i + n / 2] = p.sub(q);
      w = w.mul(wn);
    }
  }
}
