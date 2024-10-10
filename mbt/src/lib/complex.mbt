pub struct Complex {
  real : Double
  imag : Double
}

pub fn Complex::add(self : Complex, other : Complex) -> Complex {
  { real: self.real + other.real, imag: self.imag + other.imag }
}

pub fn Complex::sub(self : Complex, other : Complex) -> Complex {
  { real: self.real - other.real, imag: self.imag - other.imag }
}

pub fn Complex::mul(self : Complex, other : Complex) -> Complex {
  {
    real: self.real * other.real - self.imag * other.imag,
    imag: self.real * other.imag + self.imag * other.real,
  }
}

pub fn Complex::mul_scalar(self : Complex, scalar : Double) -> Complex {
  { real: self.real * scalar, imag: self.imag * scalar }
}
