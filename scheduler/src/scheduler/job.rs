use rand::Rng;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Status {
  Pending,
  Running,
  Completed,
  Failed,
}

#[derive(Debug, Clone)]
pub struct Job{
  id: String,
  gpu_required: u32,
  memory_required: u32, 
  runtime: u32,
  status: Status,
  remaining_time: u32,
}

impl Job {
  pub fn new(id: String, gpu_required: u32, memory_required:u32, runtime: u32) -> Self {
    Self {
      id,
      gpu_required,
      memory_required,
      runtime, 
      status: Status::Pending, 
      remaining_time: runtime,
    }
  }

  pub fn id(&self) -> &str {
    &self.id
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

  pub fn start_job(&mut self) {
    self.status = Status::Running;
  }

  pub fn remaining_time(&self) -> u32 {
    self.remaining_time
  }

  pub fn decrement_remaining_time(&mut self, time: u32) {
    if time >= self.remaining_time {
      self.remaining_time = 0;
      self.status = Status::Completed;
    } else {
      self.remaining_time -= time;
    }
  }

  pub fn new_random() -> Self {
    let mut rng = rand::thread_rng();
    let id = Uuid::new_v4().to_string();
    let gpu_required = rng.gen_range(1..5);
    let memory_required = rng.gen_range(4..33); //in GB
    let runtime = rng.gen_range(5..21); //in simulation steps

    Self::new(id, gpu_required, memory_required, runtime)
  }

}