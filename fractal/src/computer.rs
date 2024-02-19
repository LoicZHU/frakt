pub mod julia;

use complex::Complex;

pub trait FractalComputer {
  fn max_iterations(&self) -> u32;
  fn fractal_param(&self) -> Complex;
  fn divergence_threshold_square(&self) -> f32;
  fn fractal_function(z: Complex, c: Complex) -> Complex;
  fn has_converged(&self, zn: &Complex) -> bool;
  fn zn_computer(&self, zn: &Complex) -> f32;

  fn compute_point(&self, complex_point: Complex) -> (f32, f32) {
    let mut i = 0;
    let mut zn = complex_point;

    while i < self.max_iterations() {
      zn = Self::fractal_function(zn, self.fractal_param());

      if self.has_converged(&zn) {
        break;
      }
      i += 1;
    }

    (
      self.zn_computer(&zn),
      i as f32 / self.max_iterations() as f32,
    )
  }
}
