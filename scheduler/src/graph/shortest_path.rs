use std::collections::HashMap;

use petgraph::algo::dijkstra;
use petgraph::graph::{UnGraph, NodeIndex};

use crate::graph::datacenter::{GraphNode, LinkWeight, NodeType};

pub fn find_shortest_path(g: &UnGraph<GraphNode, LinkWeight>, source: NodeIndex) -> HashMap<NodeIndex, u32>{
  let node_map = dijkstra(g, source, None, |edge| edge.weight().get_weight());
  return node_map;
}

pub fn servers_sorted_by_distance(g: &UnGraph<GraphNode, LinkWeight>, node_map: &HashMap<NodeIndex, u32>) -> Vec<NodeIndex> {
  let mut res: Vec<_> = node_map.iter().collect();
  res.sort_by(|a, b| a.1.cmp(b.1));
  res.into_iter()
    .map(|(node, _)| *node)
    .filter(|node| {
      match &g[*node].kind(){
        NodeType::Server => true,
        _ => false,
      }
    })
    .collect()
}