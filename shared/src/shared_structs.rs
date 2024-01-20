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
pub struct ComplexOld {
  pub re: f64,
  pub im: f64,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub enum FractalDescriptor {
  Julia(JuliaDescriptor),
  IteratedSinZ(IteratedSinZDescriptor),
  Mandelbrot(MandelbrotDescriptor),
  NewtonRaphsonZ3(NewtonRaphsonZ3Descriptor),
  NewtonRaphsonZ4(NewtonRaphsonZ4Descriptor),
  NovaNewtonZ3(NovaNewtonRaphsonZ3Descriptor),
  NovaNewtonZ4(NovaNewtonRaphsonZ4Descriptor),
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct IteratedSinZDescriptor {
  pub c: ComplexOld,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct JuliaDescriptor {
  pub c: ComplexOld,
  pub divergence_threshold_square: f64,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct MandelbrotDescriptor {}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct NewtonRaphsonZ3Descriptor {}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct NewtonRaphsonZ4Descriptor {}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct NovaNewtonRaphsonZ3Descriptor {}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct NovaNewtonRaphsonZ4Descriptor {}

#[derive(Debug, Deserialize, Serialize)]
pub struct PixelData {
  pub offset: u32,
  pub count: u32,
}

pub struct PixelIntensity {
  pub zn: f32,
  pub count: f32,
}

impl PixelIntensity {
  pub fn new(zn: f32, count: f32) -> PixelIntensity {
    PixelIntensity { zn, count }
  }
}

// add the other structs here //
