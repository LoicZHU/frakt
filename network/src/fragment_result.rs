use fractal::generator::{Point, Range, Resolution};
use serde::Deserialize;
use serde::Serialize;
use serde_json::json;

#[derive(Debug, Deserialize, Serialize)]
pub struct Metadata {
  offset: u32,
  count: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FragmentResult {
  pub id: Metadata,
  pub resolution: Resolution,
  pub range: Range,
  pub pixels: Metadata,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Id {
  offset: u32,
  count: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Pixels {
  offset: u32,
  count: u32,
}

impl FragmentResult {
  pub fn builder() -> FragmentResultBuilder {
    FragmentResultBuilder {
      id: None,
      resolution: None,
      range: None,
      pixels: None,
    }
  }

  pub fn to_json(&self) -> Result<String, serde_json::Error> {
    serde_json::to_string(&json!({"FragmentResult": self}))
  }
}

pub struct FragmentResultBuilder {
  id: Option<Metadata>,
  resolution: Option<Resolution>,
  range: Option<Range>,
  pixels: Option<Metadata>,
}

impl FragmentResultBuilder {
  pub fn with_id(mut self, offset: u32, count: u32) -> Self {
    self.id = Some(Metadata { offset, count });
    self
  }

  pub fn with_resolution(mut self, width: u16, height: u16) -> Self {
    self.resolution = Some(Resolution { width, height });
    self
  }

  pub fn with_range(mut self, min_x: f64, min_y: f64, max_x: f64, max_y: f64) -> Self {
    self.range = Some(Range {
      min: Point { x: min_x, y: min_y },
      max: Point { x: max_x, y: max_y },
    });
    self
  }

  pub fn with_pixels(mut self, offset: u32, count: u32) -> Self {
    self.pixels = Some(Metadata { offset, count });
    self
  }

  pub fn build(self) -> Result<FragmentResult, &'static str> {
    let id = self.id.ok_or("Id is missing")?;
    let resolution = self.resolution.ok_or("Resolution is missing")?;
    let range = self.range.ok_or("Range is missing")?;
    let pixels = self.pixels.ok_or("Pixels is missing")?;

    Ok(FragmentResult {
      id,
      resolution,
      range,
      pixels,
    })
  }
}
