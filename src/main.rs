// use shared::{FragmentRequest, Resolution};
// use worker::Worker;

fn main() -> Result<(), ()> {
  // let mut worker = Worker::new("localhost".to_string(), "group3".to_string(), 8787);
  //
  // let request = FragmentRequest::builder()
  //   .with_max_work_load(1000)
  //   .with_worker_name("worker1".to_string())
  //   .build()?;
  //
  // let request_str = request.to_json().map_err(|err| {
  //   eprintln!("Error transforming to json string: {}", err);
  //   "Error transforming to json string"
  // })?;
  //
  // worker.run_worker(request_str);
  //
  // //#region generate all fractals locally
  // let resolution = Resolution { nx: 1280, ny: 960 };
  // let max_iterations = 110;
  // let can_generate_all_fractals_locally = false;
  //
  // if can_generate_all_fractals_locally {
  //   worker.generate_all_fractal_models_locally(&resolution, max_iterations)?;
  // }
  // //#endregion
  //
  Ok(())
}
