use shared::FragementRequest;
use worker::Worker;

fn main() {
  let mut worker: Worker = Worker::new("localhost".to_string(), "group3".to_string(), 8787);

  let request = FragementRequest::builder()
    .with_max_work_load(1000)
    .with_worker_name("worker1".to_string())
    .build()
    .unwrap();

  let request_str = request
    .to_json()
    .expect("error while transforming to json string");

  worker.run_worker(request_str);
}
