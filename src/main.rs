// use shared::{FragmentRequest, Resolution};
// use worker::Worker;

use fractal::computer::julia::JuliaComputer;
use fractal::generator::Generator;
use network::{Client, FractalDescriptor};
use std::io;

fn main() -> Result<(), io::Error> {
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
  //#endregion
  let mut client = Client::new(
    String::from("localhost"),
    String::from("test_connection"),
    None,
  );

  client.connect_to_server()?;
  client.send_fragment_request(String::from("random name"), 2000000)?;
  loop {
    let (fragment_task, fragment_task_id) = client.read_fragment_task()?;

    let FractalDescriptor::Julia(julia_descriptor) = fragment_task.fractal_descriptor;

    let julia_computer = JuliaComputer::new(
      julia_descriptor.c,
      fragment_task.max_iteration,
      julia_descriptor.divergence_threshold_square as f32,
    );
    let fractal_generator: Generator<JuliaComputer> = Generator::new(
      fragment_task.range,
      fragment_task.resolution,
      julia_computer,
    );

    println!("Generating fractals");
    let pixels = fractal_generator.generate_fractal();

    println!("Sending fragment result");
    client.connect_to_server()?;
    client.send_fragment_result(fragment_task, fragment_task_id, pixels)?;
  }
}
