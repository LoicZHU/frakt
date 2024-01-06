use std::env;

use shared::FragmentRequest;
use worker::Worker;

fn main() -> Result<(), &'static str> {
  let args: Vec<String> = env::args().collect();
  let error_message = "⚠️ Usage: worker <server_address>";

  if args.len() <= 2 {
    eprintln!("{}", error_message);
    std::process::exit(1);
  }

  let peer = String::from(&args[1]);
  if peer.to_uppercase().ne("WORKER") {
    eprintln!("{}", error_message);
    std::process::exit(1);
  }

  let server_ip = String::from(&args[2]);
  let server_port = 8787;
  let connection_name = String::from("group3");
  let mut worker = Worker::new(server_ip, server_port, connection_name);

  let request = FragmentRequest::builder()
    .with_max_work_load(1000)
    .with_worker_name(Worker::generate_random_name_with_prefix("WORKER_"))
    .build()?;

  let request_str = request.to_json().map_err(|err| {
    eprintln!("Error transforming to json string: {}", err);
    "Error transforming to json string"
  })?;

  worker.run_worker(request_str);

  Ok(())
}
