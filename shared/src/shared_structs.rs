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

#[derive(Debug, Deserialize, Serialize)]
pub enum FractalDescriptor {
  Julia(JuliaDescriptor),
  IteratedSinZ(IteratedSinZDescriptor),
  Mandelbrot(MandelbrotDescriptor),
  NewtonRaphsonZ3(NewtonRaphsonZ3Descriptor),
  NewtonRaphsonZ4(NewtonRaphsonZ4Descriptor),
  NovaNewtonZ3(NovaNewtonRaphsonZ3Descriptor),
  NovaNewtonZ4(NovaNewtonRaphsonZ4Descriptor),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct IteratedSinZDescriptor {
  pub c: Complex,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JuliaDescriptor {
  pub c: Complex,
  pub divergence_threshold_square: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MandelbrotDescriptor {}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewtonRaphsonZ3Descriptor {}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewtonRaphsonZ4Descriptor {}

#[derive(Debug, Deserialize, Serialize)]
pub struct NovaNewtonRaphsonZ3Descriptor {}

#[derive(Debug, Deserialize, Serialize)]
pub struct NovaNewtonRaphsonZ4Descriptor {}

pub struct PixelIntensity {
  pub zn: f32,
  pub count: f32,
}

// add the other structs here //
