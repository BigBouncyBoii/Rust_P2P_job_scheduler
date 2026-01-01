pub mod network;
pub mod node;
pub mod scheduler;
pub mod graph;

use crate::graph::datacenter::{build_leaf_spine_topology, NodeType};
use crate::scheduler::job::Job;
use crate::scheduler::scheduler::schedule_job;
use crate::network::network::advance_hops;
use log::info;


const SIMULATION_STEPS: u32 = 500; 

fn main() {
  env_logger::init();
  info!("Starting scheduler simulation");
  let mut g = build_leaf_spine_topology(3, 4, 2);
  let mut jobs = vec![];
  for _ in 0..100{
    let job = Job::new_random();
    jobs.push(job);
  }


  let mut in_transit_jobs = Vec::new();

  let source = g
    .node_indices()
    .find(|&i| matches!(g[i].kind(), NodeType::LeafSwitch))
    .unwrap();

  for job in jobs {
    match schedule_job(job, source, &mut g) {
        Ok(in_flight) => {
            println!(
                "Job submitted towards server {}",
                g[in_flight.destination_node].id()
            );
            if let Some(node) = g[in_flight.destination_node].node() {
                node.reserve_for_in_transit(in_flight.job.clone());
            }
            in_transit_jobs.push(in_flight);
        }
        Err(e) => {
            println!("Failed to submit job: {}", e);
        }
    }
  }

  for time_step in 0..SIMULATION_STEPS {
    println!("=== Time step: {} ===", time_step);

    advance_hops(&mut in_transit_jobs, &mut g);

    for node_index in g.node_indices() {
      if let Some(node) = g[node_index].node() {
        node.tick();
        node.try_schedule_waiting();
      }
    }
  }

}
