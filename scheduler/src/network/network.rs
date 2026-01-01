use crate::graph::datacenter::{GraphNode, LinkWeight};
use crate::scheduler::scheduler::InTransitJob;
use petgraph::graph::{UnGraph};

pub fn advance_hops(in_transit_jobs: &mut Vec<InTransitJob>, g: &mut UnGraph<GraphNode, LinkWeight>) {

  for x in in_transit_jobs.iter_mut() {
    if x.remaining_latency > 0 {
      x.remaining_latency -= 1;
    }

    if x.remaining_latency == 0 && x.hop < x.path.len() - 1 {
      x.hop += 1;
      if x.hop < x.path.len() - 1 {
        let next_edge_latency = g.find_edge(x.path[x.hop], x.path[x.hop + 1])
          .and_then(|e| g.edge_weight(e).map(|w| w.get_weight()))
          .unwrap_or(0);
        x.remaining_latency = next_edge_latency;
      }
    }
  }

  let mut arrived_indices = Vec::new();

  for (i, x) in in_transit_jobs.iter_mut().enumerate() {
    if x.remaining_latency == 0 && x.hop == x.path.len() - 1 {
      arrived_indices.push(i);
    }
  }

  if arrived_indices.len() > 0 {
    log::info!("Jobs arriving this step: {}", arrived_indices.len());
  }

  for i in arrived_indices.into_iter().rev() {
    let in_transit = in_transit_jobs.remove(i);

    if let Some(node) = g[in_transit.destination_node].node() {
      node.arrive_from_transit(in_transit.job.id());
    }
  }
  
}