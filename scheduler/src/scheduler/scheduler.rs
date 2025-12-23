use::petgraph::graph::{UnGraph, NodeIndex};

use crate::node::node::Node;
use crate::scheduler::job::Job;
use crate::graph::datacenter::{GraphNode, LinkWeight};
use crate::graph::shortest_path::{find_shortest_path, servers_sorted_by_distance};


pub fn schedule_job(mut job: Job, source: NodeIndex, g: &mut UnGraph<GraphNode, LinkWeight>) -> Result<NodeIndex, String> {
    let distances = find_shortest_path(&g, source);
    let mut candidates = servers_sorted_by_distance(&g, &distances);

    for server_index in candidates {
        let server = &mut g[server_index];
        match assign_server_job(server, job) {
            Ok(()) => return Ok(server_index), 
            Err(j) => job = j, 
        }
    }
    return Err("No available servers found".to_string());
}

fn assign_server_job(server: &mut GraphNode, job: Job) -> Result<(), Job> {
  if let Some(node) = server.node(){
    if node.node_available(&job) {
      node.add_job(job);
      return Ok(());
    }
  }
  Err(job)
}