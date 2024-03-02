use shared::{FractalDescriptor, FragmentTask, PixelIntensity};
pub trait Fractal {
  fn generate(
    &self,
    fragment_task: &FragmentTask,
    descriptor: &FractalDescriptor,
  ) -> Vec<PixelIntensity>;

  fn generate_locally();

  fn generate_graphicly(); 
}
