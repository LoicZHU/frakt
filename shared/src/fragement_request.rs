use serde::{Deserialize, Serialize};
use serde_json::Result as SerdeResult;

#[derive(Debug, Serialize, Deserialize)]
pub struct FragmentRequest {
  worker_name: String,
  maximal_work_load: u32,
}

impl FragmentRequest {
  fn new(worker_name: String, maximal_work_load: u32) -> Self {
    Self {
      worker_name,
      maximal_work_load,
    }
  }

  pub fn builder() -> FragmentRequestBuilder {
    FragmentRequestBuilder::default()
  }

  pub fn to_json(&self) -> SerdeResult<String> {
    serde_json::to_string(self)
  }
}

#[derive(Default)]
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

  pub fn build(self) -> Result<FragmentRequest, String> {
    let worker_name = self
      .worker_name
      .ok_or_else(|| "Worker name is missing".to_string())?;
    let max_work_load = self
      .max_work_load
      .ok_or_else(|| "Max work load is missing".to_string())?;

    Ok(FragmentRequest::new(worker_name, max_work_load))
  }
}
