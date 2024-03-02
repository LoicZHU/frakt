use crate::{FractalDescriptor, Range, Resolution, U8Data};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct FragmentTask {
  #[serde(rename = "id")]
  pub id: U8Data,
  pub max_iteration: u32,
  pub resolution: Resolution,
  pub range: Range,
  pub fractal: FractalDescriptor,
}

impl FragmentTask {
  pub fn new(
    id: U8Data,
    max_iteration: u32,
    resolution: Resolution,
    range: Range,
    fractal: FractalDescriptor,
  ) -> Self {
    FragmentTask {
      id,
      max_iteration,
      resolution,
      range,
      fractal,
    }
  }

  pub fn to_string(&self) -> String {
    serde_json::to_string(self).expect("Failed to transform FragmentTask to JSON")
  }
}
