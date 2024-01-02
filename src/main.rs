use shared::FragmentRequest;
use worker::Worker;

fn main() -> Result<(), &'static str> {
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

  Ok(())
}
