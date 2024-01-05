use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Range {
  pub min: Point,
  pub max: Point,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct Resolution {
  pub nx: u16,
  pub ny: u16,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct U8Data {
  pub offset: u32,
  pub count: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Point {
  pub x: f64,
  pub y: f64,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct Complex {
  pub re: f64,
  pub im: f64,
}

impl Complex {
  pub fn new(re: f64, im: f64) -> Self {
    Complex { re, im }
  }

  pub fn square_norm(&self) -> f64 {
    self.re * self.re + self.im * self.im
  }

  pub fn add(&self, other: &Complex) -> Complex {
    Complex {
      re: self.re + other.re,
      im: self.im + other.im,
    }
  }

  pub fn mul(&self, other: &Complex) -> Complex {
    Complex {
      re: self.re * other.re - self.im * other.im,
      im: self.re * other.im + self.im * other.re,
    }
  }

  pub fn sin(&self) -> Complex {
    Complex {
      re: self.re.sin() * self.im.cosh(),
      im: self.re.cos() * self.im.sinh(),
    }
  }

  pub fn square(&self) -> Complex {
    Complex {
      re: self.re * self.re - self.im * self.im,
      im: 2.0 * self.re * self.im,
    }
  }
}

#[derive(Debug, Deserialize, Serialize)]
pub enum FractalDescriptor {
  Julia(JuliaDescriptor),
  Mandelbrot(MandelbrotDescriptor),
  IteratedSinZ(IteratedSinZDescriptor),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JuliaDescriptor {
  pub c: Complex,
  pub divergence_threshold_square: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MandelbrotDescriptor {}

#[derive(Debug, Deserialize, Serialize)]
pub struct IteratedSinZDescriptor {
  pub c: Complex,
}

pub struct PixelIntensity {
  pub zn: f32,
  pub count: f32,
}

// add the other structs here //
