mod complex;

pub use complex::ComplexTrait;
use serde::{Deserialize, Serialize};
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct Complex {
  pub re: f64,
  pub im: f64,
}
impl Add for Complex {
  type Output = Self;
  fn add(self, rhs: Self) -> Self::Output {
    Complex {
      re: self.re + rhs.re,
      im: self.im + rhs.im,
    }
  }
}

impl Sub for Complex {
  type Output = Self;

  fn sub(self, rhs: Self) -> Self::Output {
    Complex {
      re: self.re - rhs.re,
      im: self.im - rhs.im,
    }
  }
}

impl Mul for Complex {
  type Output = Self;

  fn mul(self, rhs: Self) -> Self::Output {
    Complex {
      re: self.re * rhs.re - self.im * rhs.im,
      im: self.re * rhs.im + self.im * rhs.re,
    }
  }
}
impl Div for Complex {
  type Output = Self;

  fn div(self, rhs: Self) -> Self::Output {
    let divisor = rhs.re * rhs.re + rhs.im * rhs.im;

    Complex {
      re: (self.re * rhs.re + self.im * rhs.im) / divisor,
      im: (self.im * rhs.re - self.re * rhs.im) / divisor,
    }
  }
}

impl Complex {
  pub fn new(re: f64, im: f64) -> Self {
    Complex { re, im }
  }

  fn argument(&self) -> f64 {
    self.im.atan2(self.re)
  }

  fn sine(&self) -> Self {
    Complex {
      re: self.re.sin() * self.im.cosh(),
      im: self.re.cos() * self.im.sinh(),
    }
  }

  fn square_norm(&self) -> f64 {
    self.re.powf(2.0) + self.im.powf(2.0)
  }
}
