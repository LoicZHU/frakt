use complex::Complex;
use fractal::generator::{Range, Resolution};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct FragmentTaskMessage {
  #[serde(rename = "FragmentTask")]
  pub fragment_task: FragmentTask,
}

pub type FragmentTaskId = Vec<u8>;

#[derive(Debug, Deserialize, Serialize)]
pub struct U8Data {
  offset: u32,
  count: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub enum FractalDescriptor {
  Julia(JuliaDescriptor),
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct JuliaDescriptor {
  pub c: Complex,
  pub divergence_threshold_square: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FragmentTask {
  #[serde(rename = "id")]
  pub id: U8Data,
  #[serde(rename = "fractal")]
  pub fractal_descriptor: FractalDescriptor,
  pub max_iteration: u32,
  pub resolution: Resolution,
  pub range: Range,
}

impl FragmentTask {
  pub fn new(
    id: U8Data,
    fractal_descriptor: FractalDescriptor,
    max_iteration: u32,
    resolution: Resolution,
    range: Range,
  ) -> Self {
    FragmentTask {
      id,
      fractal_descriptor,
      max_iteration,
      resolution,
      range,
    }
  }

  pub fn to_string(&self) -> String {
    serde_json::to_string(self).expect("Failed to transform FragmentTask to JSON")
  }
}
