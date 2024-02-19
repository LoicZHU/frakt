use crate::computer::FractalComputer;
use complex::Complex;

pub struct JuliaComputer {
  fractal_param: Complex,
  max_iterations: u32,
  divergence_threshold_square: f32,
}

impl JuliaComputer {
  pub fn new(
    fractal_param: Complex,
    max_iterations: u32,
    divergence_threshold_square: f32,
  ) -> Self {
    JuliaComputer {
      fractal_param,
      max_iterations,
      divergence_threshold_square,
    }
  }
}

impl FractalComputer for JuliaComputer {
  fn max_iterations(&self) -> u32 {
    self.max_iterations
  }
  fn fractal_param(&self) -> Complex {
    self.fractal_param
  }

  fn divergence_threshold_square(&self) -> f32 {
    self.divergence_threshold_square
  }

  fn fractal_function(z: Complex, c: Complex) -> Complex {
    z * z + c
  }

  fn has_converged(&self, zn: &Complex) -> bool {
    zn.square_norm() < self.divergence_threshold_square()
  }

  fn zn_computer(&self, zn: &Complex) -> f32 {
    zn.square_norm() / self.divergence_threshold_square()
  }
}
