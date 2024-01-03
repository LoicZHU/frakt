use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
pub struct FragmentRequest {
  worker_name: String,
  maximal_work_load: u32,
}

impl FragmentRequest {
  fn new(worker_name: String, maximal_work_load: u32) -> FragmentRequest {
    FragmentRequest {
      worker_name: worker_name,
      maximal_work_load: maximal_work_load,
    }
  }

  pub fn builder() -> FragmentRequestBuilder {
    FragmentRequestBuilder {
      worker_name: None,
      max_work_load: None,
    }
  }

  pub fn to_json(&self) -> Result<String, serde_json::Error> {
    serde_json::to_string(&json!({"FragmentRequest": self}))
  }
}

pub struct FragmentRequestBuilder {
  worker_name: Option<String>,
  max_work_load: Option<u32>,
}

impl FragmentRequestBuilder {
  pub fn with_worker_name(mut self, worker_name: String) -> Self {
    self.worker_name = Some(worker_name);
    self
  }

  pub fn with_max_work_load(mut self, max_work_load: u32) -> Self {
    self.max_work_load = Some(max_work_load);
    self
  }

  pub fn build(self) -> Result<FragmentRequest, &'static str> {
    let worker_name = self.worker_name.ok_or("Worker name is missing")?;
    let max_work_load = self.max_work_load.ok_or("Max work load is missing")?;

    Ok(FragmentRequest::new(worker_name, max_work_load))
  }
}
