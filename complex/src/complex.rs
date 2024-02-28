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

// pub trait ComplexTrait {
//   fn new(re: f64, im: f64) -> Self;
//   fn add(&self, other: &Self) -> Self;
//   fn argument(&self) -> f64;
//   fn divide(&self, other: Self) -> Self;
//   fn multiply(&self, other: &Self) -> Self;
//   fn sine(&self) -> ComplexOld;
//   fn square(&self) -> ComplexOld;
//   fn square_norm(&self) -> f64;
//   fn subtract(&self, other: &Self) -> Self;
// }
// impl ComplexTrait for ComplexOld {
//   fn new(re: f64, im: f64) -> Self {
//     ComplexOld { re, im }
//   }
//
//   fn add(&self, other: &ComplexOld) -> ComplexOld {
//     ComplexOld {
//       re: self.re + other.re,
//       im: self.im + other.im,
//     }
//   }
//
//   fn argument(&self) -> f64 {
//     self.im.atan2(self.re)
//   }
//
//   fn divide(&self, other: ComplexOld) -> ComplexOld {
//     let divisor = other.re * other.re + other.im * other.im;
//     ComplexOld {
//       re: (self.re * other.re + self.im * other.im) / divisor,
//       im: (self.im * other.re - self.re * other.im) / divisor,
//     }
//   }
//
//   fn multiply(&self, other: &ComplexOld) -> ComplexOld {
//     ComplexOld {
//       re: self.re * other.re - self.im * other.im,
//       im: self.re * other.im + self.im * other.re,
//     }
//   }
//
//   fn sine(&self) -> ComplexOld {
//     ComplexOld {
//       re: self.re.sin() * self.im.cosh(),
//       im: self.re.cos() * self.im.sinh(),
//     }
//   }
//
//   fn square(&self) -> ComplexOld {
//     ComplexOld {
//       re: self.re * self.re - self.im * self.im,
//       im: 2.0 * self.re * self.im,
//     }
//   }
//
//   fn square_norm(&self) -> f64 {
//     self.re * self.re + self.im * self.im
//   }
//
//   fn subtract(&self, other: &ComplexOld) -> ComplexOld {
//     ComplexOld {
//       re: self.re - other.re,
//       im: self.im - other.im,
//     }
//   }
// }
