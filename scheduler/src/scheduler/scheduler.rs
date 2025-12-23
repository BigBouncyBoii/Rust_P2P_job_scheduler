
use crate::node::node::Node;
use crate::scheduler::job::Job;


pub fn schedule_job(job: Job, source: NodeIndex, graph: &mut Graph) {
    let distances = find_shortest_path(&graph, source);
    let mut candidates = leaf_nodes_sorted_by_distance(distances);

    let mut job = job;
    for server_index in candidates {
        let server = &mut graph[server_index];
        match assign_server_job(server, job) {
            Ok(()) => return, 
            Err(j) => job = j, 
        }
    }
    graph[source].enqueue_job(job);
}

fn assign_server_job(server: &mut Node, job: Job) -> Result<(), Job> {
  if server.node_available(&job){
    server.add_job(job);
    return Ok(());
  }
  Err(job)
}