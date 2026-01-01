use std::collections::HashMap;

use petgraph::algo::dijkstra;
use petgraph::graph::{UnGraph, NodeIndex};

use crate::graph::datacenter::{GraphNode, LinkWeight, NodeType};

pub fn find_shortest_path(g: &UnGraph<GraphNode, LinkWeight>, source: NodeIndex) -> HashMap<NodeIndex, u32>{
  let node_map = dijkstra(g, source, None, |edge| edge.weight().get_weight());
  return node_map;
}

pub fn find_shortest_path_destination(g: &UnGraph<GraphNode, LinkWeight>, source: NodeIndex, destination: NodeIndex) -> u32{
  let node_map = dijkstra(g, source, Some(destination), |edge| edge.weight().get_weight());
  return *node_map.get(&destination).unwrap_or(&0);
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

pub fn compute_path_dijkstra(g: &UnGraph<GraphNode, LinkWeight>, source: NodeIndex, destination: NodeIndex)
  -> Result<Vec<NodeIndex>, String>
{

  let node_map = dijkstra(g, source, Some(destination), |edge| edge.weight().get_weight());
  if node_map.contains_key(&destination) {

    let mut path = Vec::new();
    let mut current = destination;
    path.push(current);
    while current != source {

      let pred = g
        .neighbors_directed(current, petgraph::Incoming)
        .min_by_key(|&n| node_map.get(&n).unwrap_or(&u32::MAX))
        .ok_or("Failed to reconstruct path")?;
      current = pred;
      path.push(current);
    }
    path.reverse();
    Ok(path)
  } else {
    Err("No path found".to_string())
  }
}