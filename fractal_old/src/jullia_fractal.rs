use crate::Fractal;
use complex::ComplexTrait;
use shared::{ComplexOld, FractalDescriptor, FragmentTask, PixelIntensity};

pub struct FractalJulia {}

impl FractalJulia {
  pub fn new() -> FractalJulia {
    FractalJulia {}
  }
}

impl Fractal for FractalJulia {
  fn generate(
    &self,
    fragment_task: &FragmentTask,
    descriptor: &FractalDescriptor,
  ) -> Vec<PixelIntensity> {
    let x_start = fragment_task.range.min.x;
    let x_end = fragment_task.range.max.x;

    let y_start = fragment_task.range.min.y;
    let y_end = fragment_task.range.max.y;

    let x_step = ((x_start - x_end) / fragment_task.resolution.nx as f64).abs();
    let y_step = ((y_start - y_end) / fragment_task.resolution.ny as f64).abs();

    let mut pixel_intensity_vec: Vec<PixelIntensity> = Vec::new();

    let mut x = x_start;
    let mut y = y_start;

    while y < y_end {
      while x < x_end {
        let pixel_complexe = ComplexOld { re: x, im: y };
        if let FractalDescriptor::Julia(julia_descriptor) = descriptor {
          let mut zn: ComplexOld = pixel_complexe;
          let mut count = 0;

          while count < fragment_task.max_iteration
            && zn.square_norm() < julia_descriptor.divergence_threshold_square
          {
            zn = zn.multiply(&zn).add(&julia_descriptor.c);
            count += 1;
          }
          let zn = zn.argument() as f32 / julia_descriptor.divergence_threshold_square as f32;
          let count = count as f32 / fragment_task.max_iteration as f32;
          pixel_intensity_vec.push(PixelIntensity::new(zn, count));
        }

        x += x_step;
      }
      x = x_start;
      y += y_step;
    }

    pixel_intensity_vec
  }

  fn generate_locally() {
    // Implementation for generate_locally goes here
  }
}
