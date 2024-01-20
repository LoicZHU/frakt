use shared::ComplexOld;

pub trait ComplexTrait {
  fn new(re: f64, im: f64) -> Self;
  fn add(&self, other: &Self) -> Self;
  fn argument(&self) -> f64;
  fn divide(&self, other: Self) -> Self;
  fn multiply(&self, other: &Self) -> Self;
  fn sine(&self) -> ComplexOld;
  fn square(&self) -> ComplexOld;
  fn square_norm(&self) -> f64;
  fn subtract(&self, other: &Self) -> Self;
}
impl ComplexTrait for ComplexOld {
  fn new(re: f64, im: f64) -> Self {
    ComplexOld { re, im }
  }

  fn add(&self, other: &ComplexOld) -> ComplexOld {
    ComplexOld {
      re: self.re + other.re,
      im: self.im + other.im,
    }
  }

  fn argument(&self) -> f64 {
    self.im.atan2(self.re)
  }

  fn divide(&self, other: ComplexOld) -> ComplexOld {
    let divisor = other.re * other.re + other.im * other.im;
    ComplexOld {
      re: (self.re * other.re + self.im * other.im) / divisor,
      im: (self.im * other.re - self.re * other.im) / divisor,
    }
  }

  fn multiply(&self, other: &ComplexOld) -> ComplexOld {
    ComplexOld {
      re: self.re * other.re - self.im * other.im,
      im: self.re * other.im + self.im * other.re,
    }
  }

  fn sine(&self) -> ComplexOld {
    ComplexOld {
      re: self.re.sin() * self.im.cosh(),
      im: self.re.cos() * self.im.sinh(),
    }
  }

  fn square(&self) -> ComplexOld {
    ComplexOld {
      re: self.re * self.re - self.im * self.im,
      im: 2.0 * self.re * self.im,
    }
  }

  fn square_norm(&self) -> f64 {
    self.re * self.re + self.im * self.im
  }

  fn subtract(&self, other: &ComplexOld) -> ComplexOld {
    ComplexOld {
      re: self.re - other.re,
      im: self.im - other.im,
    }
  }
}
