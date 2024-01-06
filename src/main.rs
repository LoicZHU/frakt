use shared::{Complex, FractalDescriptor, FragmentRequest, IteratedSinZDescriptor, JuliaDescriptor, MandelbrotDescriptor, NovaNewtonRaphsonZ3Descriptor, Point, Range, Resolution};
use worker::Worker;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let mut worker = Worker::new("localhost".to_string(), "group3".to_string(), 8787);

  let request = FragmentRequest::builder()
    .with_max_work_load(1000)
    .with_worker_name("worker1".to_string())
    .build()?;

  let request_str = request.to_json().map_err(|err| {
    eprintln!("Error transforming to json string: {}", err);
    "Error transforming to json string"
  })?;

  worker.run_worker(request_str);

  let max_iterations = 110;
  let resolution = Resolution { nx: 1280, ny: 960 };

  let range = Range {
    min: Point { x: -4.0, y: -3.0 },
    max: Point { x: 4.0, y: 3.0 },
  };

  let julia_descriptor_1 = JuliaDescriptor {
    c: Complex {
      re: 0.285,
      im: 0.013,
    },
    divergence_threshold_square: 4.0,
  };
  let julia_descriptor_2 = JuliaDescriptor {
    c: Complex {
      re: -0.9,
      im: 0.27015,
    },
    divergence_threshold_square: 4.0,
  };

  worker.generate_fractal_locally(
    &resolution,
    &range,
    FractalDescriptor::Julia(julia_descriptor_2),
    max_iterations,
  )?;

  let mandelbrot_range = Range {
    min: Point { x: -2.0, y: -1.25 },
    max: Point { x: 1.0, y: 1.25 },
  };
  worker.generate_fractal_locally(
    &resolution,
    &mandelbrot_range,
    FractalDescriptor::Mandelbrot(MandelbrotDescriptor {}),
    max_iterations,
  )?;

  let sin_z_descriptor_1 = IteratedSinZDescriptor {
    c: Complex { re: 1.0, im: 0.3 },
  };
  let sin_z_descriptor_2 = IteratedSinZDescriptor {
    c: Complex { re: 0.2, im: 1.0 },
  };
  worker.generate_fractal_locally(
    &resolution,
    &range,
    FractalDescriptor::IteratedSinZ(sin_z_descriptor_2),
    max_iterations,
  )?;

  worker.generate_fractal_locally(&resolution, &range, FractalDescriptor::NovaNewtonZ3(NovaNewtonRaphsonZ3Descriptor{}), max_iterations)?;

  Ok(())
}
