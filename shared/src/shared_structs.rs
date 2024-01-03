use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Range {
  pub min: Point,
  pub max: Point,
}

#[derive(Debug, Deserialize, Serialize)]
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

#[derive(Debug, Deserialize, Serialize)]
pub struct Complex {
  pub re: f64,
  pub im: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum FractalDescriptor {
  Julia(JuliaFractal),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JuliaFractal {
  pub c: Complex,
  pub divergence_threshold_square: f64,
}

// add the other structs here //
