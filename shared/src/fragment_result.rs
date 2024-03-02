use crate::shared_structs::PixelData;
use crate::Point;
use crate::Range;
use crate::Resolution;
use crate::U8Data;
use serde::Deserialize;
use serde::Serialize;
use serde_json::json;

#[derive(Debug, Deserialize, Serialize)]
pub struct FragmentResult {
  pub id: U8Data,
  pub resolution: Resolution,
  pub range: Range,
  pub pixels: PixelData,
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
  id: Option<U8Data>,
  resolution: Option<Resolution>,
  range: Option<Range>,
  pixels: Option<PixelData>,
}

impl FragmentResultBuilder {
  pub fn with_id(mut self, offset: u32, count: u32) -> Self {
    self.id = Some(U8Data { offset, count });
    self
  }

  pub fn with_resolution(mut self, nx: u16, ny: u16) -> Self {
    self.resolution = Some(Resolution { nx, ny });
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
    self.pixels = Some(PixelData { offset, count });
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
