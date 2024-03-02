use crate::Fractal;
use complex::ComplexTrait;
use shared::{Complex, FractalDescriptor, FragmentTask, PixelIntensity};

pub struct FractalMandelbrot {}

impl FractalMandelbrot {
  pub fn new() -> FractalMandelbrot {
    FractalMandelbrot {}
  }
}

impl Fractal for FractalMandelbrot {
  fn generate(&self, task: &FragmentTask, _descriptor: &FractalDescriptor) -> Vec<PixelIntensity> {
    let x_start = task.range.min.x;
    let x_end = task.range.max.x;
    let y_start = task.range.min.y;
    let y_end = task.range.max.y;

    let x_step = ((&x_start - &x_end) / task.resolution.nx as f64).abs();
    let y_step = ((&y_start - &y_end) / task.resolution.ny as f64).abs();

    let mut pixel_intensity_vec: Vec<PixelIntensity> = Vec::new();
    let max_iteration = task.max_iteration;

    let mut x = x_start;
    let mut y = y_start;

    while y < y_end {
      while x < x_end {
        let c = Complex::new(x, y);
        let mut zn = Complex::new(0.0, 0.0);
        let mut count = 0;

        while zn.square_norm() < 4.0 && count < max_iteration {
          zn = zn.multiply(&zn).add(&c);
          count += 1;
        }

        let intensity = count as f32 / max_iteration as f32;
        let escape_time = zn.square_norm() as f32 / 4.0;

        pixel_intensity_vec.push(PixelIntensity::new(escape_time, intensity));
        x += x_step;
      }
      x = x_start;
      y += y_step;
    }

    pixel_intensity_vec
  }

  fn generate_locally() {
    todo!("not implemented yet");
  }

  fn generate_graphicly() {
    todo!();
  }
}
