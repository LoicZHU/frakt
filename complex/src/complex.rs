use serde::{Deserialize, Serialize};
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Complex {
  pub re: f32,
  pub im: f32,
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
  pub fn new(re: f32, im: f32) -> Self {
    Complex { re, im }
  }

  pub fn argument(&self) -> f32 {
    self.im.atan2(self.re)
  }

  pub fn sine(&self) -> Self {
    Complex {
      re: self.re.sin() * self.im.cosh(),
      im: self.re.cos() * self.im.sinh(),
    }
  }

  pub fn square_norm(&self) -> f32 {
    self.re.powf(2.0) + self.im.powf(2.0)
  }
}
