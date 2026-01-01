use std::collections::VecDeque;

use crate::scheduler::job::Job;

const MAX_GPU_QUEUED_JOBS: u32 = 100;
const MAX_MEMORY_QUEUED_JOBS: u32 = 100;

#[derive(Debug, Clone)]
pub struct Node {
  id: String,
  available_gpus: u32,
  available_memory: u32,
  running_jobs: Vec<Job>,
  waiting_jobs: VecDeque<Job>,
  in_transit_jobs: Vec<Job>,  // Jobs scheduled to this node but not yet arrived
}

impl Node {

  pub fn new(id: String, available_gpus: u32, available_memory: u32) -> Self {
    Self {
      id,
      available_gpus,
      available_memory,
      running_jobs: Vec::new(),
      waiting_jobs: VecDeque::new(),
      in_transit_jobs: Vec::new(),
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

  pub fn running_jobs(&self) -> &[Job] { //slice is just a pointer 
    &self.running_jobs
  }

  pub fn waiting_jobs(&self) -> &VecDeque<Job> {
    &self.waiting_jobs
  }

  pub fn node_available(&self, curr_job: &Job) -> bool {
    return self.leftover_memory(curr_job.memory_required()) && self.leftover_gpu(curr_job.gpu_required());
  }

  pub fn node_available_with_running_resources(&self, curr_job: &Job) -> bool {
    let running_memory: u32 = self.running_jobs.iter().map(|j| j.memory_required()).sum();
    let running_gpus: u32 = self.running_jobs.iter().map(|j| j.gpu_required()).sum();
    return (running_memory + curr_job.memory_required() <= self.available_memory) &&
           (running_gpus + curr_job.gpu_required() <= self.available_gpus);
  }

  fn leftover_memory(&self, curr: u32) -> bool {
    MAX_MEMORY_QUEUED_JOBS >= self.used_memory() + curr 
  }

  fn leftover_gpu(&self, curr: u32) -> bool {
    MAX_GPU_QUEUED_JOBS >= self.used_gpus() + curr
  }

  pub fn used_memory(&self) -> u32 {
    let running: u32 = self.running_jobs.iter().map(|j| j.memory_required()).sum();
    let waiting: u32 = self.waiting_jobs.iter().map(|j| j.memory_required()).sum();
    let in_transit: u32 = self.in_transit_jobs.iter().map(|j| j.memory_required()).sum();
    running + waiting + in_transit
  }

  pub fn used_gpus(&self) -> u32 {
    let running: u32 = self.running_jobs.iter().map(|j| j.gpu_required()).sum();
    let waiting: u32 = self.waiting_jobs.iter().map(|j| j.gpu_required()).sum();
    let in_transit: u32 = self.in_transit_jobs.iter().map(|j| j.gpu_required()).sum();
    running + waiting + in_transit
  }


  pub fn add_running_job(&mut self, job: Job) {
    log::info!("Job {} started on node {}", job.id(), self.id());
    self.running_jobs.push(job);
  }

  pub fn add_waiting_job(&mut self, job: Job) {
    log::info!("Job {} queued on node {} (waiting_count={})", job.id(), self.id(), self.waiting_jobs.len() + 1);
    self.waiting_jobs.push_back(job);
  }

  pub fn reserve_for_in_transit(&mut self, job: Job) {
    log::info!("Job {} reserved on node {} (in_transit)", job.id(), self.id());
    self.in_transit_jobs.push(job);
  }

  pub fn arrive_from_transit(&mut self, job_id: &str) {
    if let Some(pos) = self.in_transit_jobs.iter().position(|j| j.id() == job_id) {
      let job = self.in_transit_jobs.remove(pos);
      log::info!("Job {} arrived at node {}, moving to waiting queue", job.id(), self.id());
      self.waiting_jobs.push_back(job);
    }
  }

  pub fn tick(&mut self){
    let self_id = self.id.clone();
    let mut new_running = Vec::new();
    for job in self.running_jobs.drain(..){
      let mut job = job;
      job.decrement_remaining_time(1);
      if job.remaining_time() > 0 {
        new_running.push(job);
      } else {
        log::info!("Job {} completed on node {}", job.id(), self_id);
      }
    }
    self.running_jobs = new_running;
  }

  pub fn try_schedule_waiting(&mut self) {
    while let Some(job) = self.waiting_jobs.front() {
          if self.node_available_with_running_resources(job) {
        let job = self.waiting_jobs.pop_front().unwrap();
          log::info!("Moving job {} from waiting to running on node {}", job.id(), self.id());
          self.running_jobs.push(job);
      } else {
          break;
      }
    }
  } 

}