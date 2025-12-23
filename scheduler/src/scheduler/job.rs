
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Status {
  Pending,
  Running,
  Completed,
  Failed

}

#[derive(Debug, Clone)]
pub struct Job{
  id: String,
  message: String,
  gpu_required: u32,
  memory_required: u32, 
  runtime: u32,
  status: Status,
}

impl Job {
  pub fn new(id: String, message: String, gpu_required: u32, memory_required:u32, runtime: u32, status: Status) -> Self {
    Self {
      id,
      message,
      gpu_required,
      memory_required,
      runtime, 
      status, 
    }
  }

  pub fn id(&self) -> &str {
    &self.id
  }

  pub fn message(&self) -> &str {
    &self.message
  }

  pub fn gpu_required(&self) -> u32 {
    self.gpu_required
  }

  pub fn memory_required(&self) -> u32 {
    self.memory_required
  }
  
  pub fn runtime(&self) -> u32 {
    self.runtime
  }

  pub fn status(&self) -> &Status {
    &self.status
  }


}