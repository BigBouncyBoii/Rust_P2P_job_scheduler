use crate::scheduler::job::Job;

pub struct Node {
  id: u32,
  available_gpus: u32,
  available_memory: u32,
  jobs: Vec<Job>,
}

impl Node {

  pub fn new(id: u32, available_gpus: u32, available_memory: u32) -> Self {
    Self {
      id,
      available_gpus,
      available_memory,
      jobs: Vec::new(),
    }
  }

  pub fn id(&self) -> u32{
    self.id
  }

  pub fn available_gpus(&self) -> u32 {
    self.available_gpus
  }

  pub fn available_memory(&self) -> u32 {
    self.available_memory
  }

  pub fn jobs(&self) -> &[Job] { //slice is just a pointer 
    &self.jobs
  }
}