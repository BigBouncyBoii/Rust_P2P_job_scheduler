use crate::scheduler::job::Job;
use crate::graph::datacenter::{GraphNode, LinkWeight};
use petgraph::graph::{NodeIndex, UnGraph};
use crate::graph::shortest_path::{find_shortest_path, servers_sorted_by_distance, compute_path_dijkstra};


pub struct InTransitJob {
  pub job: Job,
  pub path: Vec<NodeIndex>,
  pub hop: usize,
  pub remaining_latency: u32,
  pub destination_node: NodeIndex,
}


pub fn schedule_job(job: Job, source: NodeIndex, g: &mut UnGraph<GraphNode, LinkWeight>) 
  -> Result<InTransitJob, String> 
{
  let distances = find_shortest_path(&g, source);
  let candidates = servers_sorted_by_distance(&g, &distances);

  for &server_index in candidates.iter() {
    let server = &mut g[server_index];

    if let Some(node) = server.node().as_ref() {
      if node.node_available(&job) {
        let path = compute_path_dijkstra(&g, source, server_index)?;

        let first_edge_latency = edge_latency(&g, path[0], path[1]);

        let in_flight = InTransitJob {
          job,
          path: path.clone(),
          hop: 0,
          remaining_latency: first_edge_latency,
          destination_node: server_index,
        };
        log::info!("Job {} scheduled: path_len={}, first_latency={}", 
          in_flight.job.id(), path.len(), first_edge_latency);
        return Ok(in_flight);
      }
    }
  }

  Err("No available servers found".to_string())
}

fn edge_latency(g: &UnGraph<GraphNode, LinkWeight>, src: NodeIndex, dst: NodeIndex) -> u32 {
  g.find_edge(src, dst)
    .and_then(|e| g.edge_weight(e).map(|w| w.get_weight()))
    .unwrap_or(1)
}
