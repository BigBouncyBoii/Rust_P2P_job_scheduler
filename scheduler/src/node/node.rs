use crate::scheduler::job::Job;

#[derive(Debug, Clone)]
pub struct Node {
  id: String,
  available_gpus: u32,
  available_memory: u32,
  jobs: Vec<Job>,
}

impl Node {

  pub fn new(id: String, available_gpus: u32, available_memory: u32) -> Self {
    Self {
      id,
      available_gpus,
      available_memory,
      jobs: Vec::new(),
    }
  }

  pub fn id(&self) -> &str{
    &self.id
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

  pub fn node_available(&self, curr_job: &Job) -> bool {
    return self.leftover_memory(curr_job.memory_required()) && self.leftover_gpu(curr_job.gpu_required());
  }

  fn leftover_memory(&self, curr_memory: u32) -> bool {
    let mut total_memory = self.available_memory();
    for job in self.jobs(){
      total_memory -= job.memory_required()
    }
    return total_memory >= curr_memory;
  }

  fn leftover_gpu(&self, curr_gpu: u32) -> bool {
    let mut total_gpu = self.available_gpus();
    for job in self.jobs(){
      total_gpu -= job.gpu_required()
    }
    return total_gpu >= curr_gpu;
  }

  pub fn add_job(&mut self, job: Job) {
    self.jobs.push(job);
  }

}