use petgraph::graph::{Graph, UnGraph};
use uuid::Uuid;
use rand::Rng;
use petgraph::graph::NodeIndex;

#[derive(Debug, Clone)]
pub enum NodeType {
  Server,
  LeafSwitch,
  SpineSwitch,
}

#[derive(Debug, Clone)]
pub struct GraphNode {
  id: String,        
  kind: NodeType,   
}

pub struct LinkWeight {
  latency_ms: u32,
  bandwidth_gbps: u32,
}

pub fn build_leaf_spine_topology(num_leaf: u32, servers_per_leaf: u32, num_spine: u32) -> UnGraph<GraphNode, LinkWeight> {
  //add stuff here
  //need to create a function which returns a petgraph of leaf and spine switches 
  let mut g = UnGraph::<GraphNode, LinkWeight>::new_undirected();
  let mut leaf_nodes: Vec<NodeIndex> = Vec::new();
  let mut spine_nodes: Vec<NodeIndex> = Vec::new();
  let mut rng = rand::thread_rng();

  build_leaf_server(num_leaf, servers_per_leaf, &mut g, &mut rng, &mut leaf_nodes);
  build_leaf_spine(num_spine, &mut g, &mut rng, &leaf_nodes, &mut spine_nodes);

  g
} 


fn build_leaf_server(num_leaf: u32, 
  servers_per_leaf: u32, 
  g: &mut UnGraph<GraphNode, LinkWeight>, 
  rng: &mut impl Rng, 
  leaf_nodes: &mut Vec<NodeIndex>) {

  for i in 0..num_leaf{
    let leaf = GraphNode{
      id: Uuid::new_v4().to_string(),
      kind: NodeType::LeafSwitch,
    };
    let leaf_node = g.add_node(leaf);
    leaf_nodes.push(leaf_node);
    for j in 0..servers_per_leaf{
      let server = GraphNode{
        id: Uuid::new_v4().to_string(),
        kind: NodeType::Server,
      };
      let server_node = g.add_node(server);
      let link_weight = LinkWeight{
        latency_ms: rng.gen_range(0..10),
        bandwidth_gbps: rng.gen_range(0..10)
      };
      g.add_edge(leaf_node, server_node, link_weight);
    }
  }
}

fn build_leaf_spine(num_spine: u32,
  g: &mut UnGraph<GraphNode, LinkWeight>,
  rng: &mut impl Rng,
  leaf_nodes: &Vec<NodeIndex>,
  spine_nodes: &mut Vec<NodeIndex>){

  for i in 0..num_spine{
    let spine = GraphNode{
      id: Uuid::new_v4().to_string(),
      kind: NodeType::SpineSwitch,
    };
    let spine_node = g.add_node(spine);
    spine_nodes.push(spine_node);
  }

  for leaf_node in leaf_nodes{
    for spine_node in &*spine_nodes{
      let link_weight = LinkWeight{
        latency_ms: rng.gen_range(0..10),
        bandwidth_gbps: rng.gen_range(0..10)
      };
      g.add_edge(*leaf_node, *spine_node, link_weight);
    }
  }
}