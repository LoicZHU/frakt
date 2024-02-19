use crate::computer::FractalComputer;
use complex::Complex;

#[derive(Debug, Copy, Clone)]
pub struct Resolution {
  pub width: u16,
  pub height: u16,
}

#[derive(Debug, Copy, Clone)]
pub struct Point {
  pub x: f64,
  pub y: f64,
}

#[derive(Debug, Copy, Clone)]
pub struct Range {
  pub min: Point,
  pub max: Point,
}

const EPSILON: f32 = 10e6;
struct Generator<T: FractalComputer> {
  range: Range,
  resolution: Resolution,
  max_iterations: u32,
  fractal_computer: T,
}

impl<T: FractalComputer> Generator<T> {
  pub fn new(
    range: Range,
    resolution: Resolution,
    max_iterations: u32,
    fractal_computer: T,
  ) -> Self {
    Self {
      range,
      resolution,
      max_iterations,
      fractal_computer,
    }
  }

  pub fn generate_fractal(&self) -> Vec<(f32, f32)> {
    let mut fractal_point: (f32, f32);
    let mut fractal_points: Vec<(f32, f32)> = Vec::new();

    let (step_x, step_y): (f32, f32) = (
      Self::calculate_step(self.range.min.x, self.range.max.x, &self.resolution.width),
      Self::calculate_step(self.range.min.y, self.range.max.y, &self.resolution.height),
    );
    let mut x: f32 = 0.;
    let mut y: f32 = 0.;
    let mut physical_point = Complex::new(x.clone(), y.clone());

    // using this form to deal with floating point numbers infinite loop problem
    while self.resolution.width as f32 - x > EPSILON {
      while self.resolution.height as f32 - y > EPSILON {
        fractal_point = self.fractal_computer.compute_point(physical_point);
        fractal_points.push(fractal_point);

        x += step_x;
        physical_point.re = x.clone();
      }
      y += step_y;
      physical_point.im = y.clone();
    }

    fractal_points
  }

  fn calculate_step(min_coordinate: f64, max_coordinate: f64, resolution_dimension: &u16) -> f32 {
    ((max_coordinate - min_coordinate) / resolution_dimension.clone() as f64) as f32
  }
}
