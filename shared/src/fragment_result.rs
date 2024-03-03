use crate::shared_structs::PixelData;
use crate::Point;
use crate::Range;
use crate::Resolution;
use crate::U8Data;
use serde::{Deserialize, Serialize};
use serde_json::Result as SerdeResult;

#[derive(Debug, Deserialize, Serialize)]
pub struct FragmentResult {
  pub id: U8Data,
  pub resolution: Resolution,
  pub range: Range,
  pub pixels: PixelData,
}

impl FragmentResult {
  pub fn builder() -> FragmentResultBuilder {
    FragmentResultBuilder::default()
  }

  pub fn to_json(&self) -> SerdeResult<String> {
    serde_json::to_string(self)
  }
}

#[derive(Default)]
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

  pub fn build(self) -> Result<FragmentResult, String> {
    let id = self.id.ok_or_else(|| "Id is missing".to_string())?;
    let resolution = self
      .resolution
      .ok_or_else(|| "Resolution is missing".to_string())?;
    let range = self.range.ok_or_else(|| "Range is missing".to_string())?;
    let pixels = self.pixels.ok_or_else(|| "Pixels is missing".to_string())?;

    Ok(FragmentResult {
      id,
      resolution,
      range,
      pixels,
    })
  }
}
