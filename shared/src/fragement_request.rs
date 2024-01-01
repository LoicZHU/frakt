use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
pub struct FragementRequest {
  worker_name: String,
  maximal_work_load: u32,
}

impl FragementRequest {
  fn new(worker_name: String, maximal_work_load: u32) -> FragementRequest {
    FragementRequest {
      worker_name: worker_name,
      maximal_work_load: maximal_work_load,
    }
  }

  pub fn builder() -> FragementRequestBuilder {
    FragementRequestBuilder {
      worker_name: None,
      max_work_load: None,
    }
  }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&json!({"FragmentRequest": self}))
    }
}

pub struct FragementRequestBuilder {
  worker_name: Option<String>,
  max_work_load: Option<u32>,
}

impl FragementRequestBuilder {
  pub fn with_worker_name(mut self, worker_name: String) -> Self {
    self.worker_name = Some(worker_name);
    self
  }

  pub fn with_max_work_load(mut self, max_work_load: u32) -> Self {
    self.max_work_load = Some(max_work_load);
    self
  }

  pub fn build(self) -> Result<FragementRequest, &'static str> {
    let worker_name = self.worker_name.ok_or("Worker name is missing")?;
    let max_work_load = self.max_work_load.ok_or("Max work load is missing")?;

    Ok(FragementRequest::new(worker_name, max_work_load))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_fragement_request_builder() {
    let fragement_request: FragementRequest = FragementRequest::builder()
      .with_worker_name("Worker1".to_string())
      .with_max_work_load(10)
      .build()
      .unwrap();

    assert_eq!(fragement_request.worker_name, "Worker1");
    assert_eq!(fragement_request.maximal_work_load, 10);

    let json_string = fragement_request.to_json().unwrap();
    println!("{}", json_string);
  }

  #[test]
  fn test_fragement_request_builder_missing_fields() {
    let result = FragementRequest::builder().with_max_work_load(10).build();
    assert!(result.is_err());

    let result = FragementRequest::builder()
      .with_worker_name("Worker1".to_string())
      .build();
    assert!(result.is_err());
  }
}
