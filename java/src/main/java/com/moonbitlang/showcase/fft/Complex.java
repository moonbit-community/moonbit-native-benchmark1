package com.moonbitlang.showcase.fft;

public record Complex(double real, double imag) {
  public String toString() {
    return String.format("%f%+fj", real, imag);
  }

  public Complex add(Complex other) {
    return new Complex(real + other.real, imag + other.imag);
  }

  public Complex sub(Complex other) {
    return new Complex(real - other.real, imag - other.imag);
  }

  public Complex mul(Complex other) {
    return new Complex(
        real * other.real - imag * other.imag, real * other.imag + imag * other.real);
  }

  public Complex mul(double other) {
    return new Complex(real * other, imag * other);
  }
}
