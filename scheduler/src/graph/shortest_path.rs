use std::collections::HashMap;

use petgraph::algo::dijkstra;
use petgraph::graph::{UnGraph, NodeIndex};

use crate::graph::datacenter::{GraphNode, LinkWeight};


pub fn find_shortest_path(g: &UnGraph<GraphNode, LinkWeight>, source: NodeIndex) -> HashMap<NodeIndex, u32>{
  let node_map = dijkstra(g, source, None, |edge| edge.weight().get_weight());
  return node_map;
}