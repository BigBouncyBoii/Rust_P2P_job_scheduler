# Rust_P2P_job_scheduler

### **1. AI Workloads in Data Centers**

- AI training and inference workloads often run on **large clusters of GPU/CPU servers**.
- These workloads are **compute-heavy** and require careful distribution across available hardware to avoid bottlenecks.
- Jobs can vary in **resource requirements** (e.g., some need more GPUs, some need more memory).

---

### **2. The Problem**

- If jobs are naively assigned to servers, some nodes may become **overloaded while others are idle**.
- Overloaded nodes can **slow down training/inference**, wasting expensive resources.
- Traditional centralized schedulers can become a **single point of failure** or a **bottleneck** in very large clusters.

### **3. How the P2P Job Scheduler Helps**

Your project simulates a **decentralized, peer-to-peer scheduling system**:

1. **Distributed decision-making**
    - Each server (node) makes independent decisions about which jobs it can handle.
    - Reduces reliance on a single scheduler.
2. **Load balancing across servers**
    - Nodes share their available resources with neighbors using **gossip or propagation algorithms**.
    - Jobs are forwarded to the most capable or least loaded nodes.
3. **Efficient resource utilization**
    - Graph algorithms (shortest path, MST, gossip overlay) ensure jobs take the **fastest or most efficient route** to available resources.
4. **Fault tolerance and scalability**
    - Since nodes communicate P2P, the system can **adapt to node failures** or new nodes joining the cluster.
    - Scales well for large AI clusters where centralized control would be expensive or slow.

## **Week 1: Core Setup and Basic Functionality**

### **Day 1–2: Rust & Project Setup**

- Set up a new Rust project with Cargo.
- Add necessary dependencies:

```toml
[dependencies]
tokio = { version ="1", features = ["full"] }
serde = { version ="1", features = ["derive"] }
petgraph ="0.7"
rand ="0.8"

```

- Define **basic structs**: `Node`, `Job`, `Message`.
- Implement a **network graph with leaf-spine topology** using `petgraph`:
    - **Leaf nodes** = servers
    - **Spine nodes** = switches connecting leaf nodes
    - Edges can have **weights** for latency/bandwidth to simulate realistic AI data center conditions.

---

### **Day 3–4: Graph Algorithms & Scheduling Logic**

- Implement **shortest path algorithm** (Dijkstra) for job forwarding across the leaf-spine topology.
- Implement **basic job scheduling**:
    - Assign jobs locally if resources are available.
    - Forward jobs to the nearest node with sufficient resources using shortest-path computation.
- Simulate **job queueing, resource usage, and network-aware routing** (take edge weights into account).

---

### **Day 5: Networking Basics**

- Implement **async TCP/UDP communication** with `tokio`.
- Define **message serialization/deserialization** using `serde`.
- Test **sending and receiving messages** between nodes in the leaf-spine topology.

---

## **Week 2: Distributed Simulation and Enhancements**

### **Day 6–7: P2P Communication**

- Connect multiple nodes in a **peer-to-peer network** (simulated as async tasks).
- Implement **job propagation** across the leaf-spine network:
    - Nodes share their **resource availability** with neighbors (leaf-to-leaf via spine nodes).
    - Forward jobs to nodes with available capacity using network-aware routing.
- Implement **periodic gossip messages** to update neighbors efficiently.

---

### **Day 8: Advanced Scheduling & Algorithms**

- Optional: implement **MST or optimized gossip overlays** across the leaf-spine topology for more efficient propagation.
- Handle **multiple concurrent jobs**, taking into account **network costs along the paths**.
- Optionally simulate **job priorities** and preemption.

This project is a **distributed job scheduling system** for AI workloads in a simulated data center. Each server node can accept jobs, track available compute resources, and communicate with neighboring nodes to **balance workloads efficiently**. Jobs are either scheduled locally if resources permit or forwarded to less busy nodes, ensuring optimal utilization across the network.

The network is modeled as a **graph with a leaf-spine topology**, where leaf nodes represent servers and spine nodes represent switches that interconnect the leaves. Edges represent network connections and can include weights for latency or bandwidth to simulate realistic data center conditions. Graph algorithms, such as **shortest path** and **gossip overlays**, are used to propagate job and resource information efficiently across this topology. Communication between nodes is handled via **async TCP/UDP messages**, simulating a peer-to-peer distributed system.

This project highlights **distributed systems concepts**, **graph-based scheduling algorithms**, and **network programming in Rust**, with relevance to **AI data center workload management**.

Prerequisites 

## **1. Rust Programming**

- **Basics of Rust syntax**: variables, structs, enums, functions, ownership, borrowing.
- **Collections**: `Vec`, `HashMap`, etc.
- **Error handling**: `Result`, `Option`, and `unwrap`/`?`.
- **Crates & Cargo**: dependency management, using external crates.

---

## **2. Async Programming**

- **Tokio** or any async runtime in Rust.
- `async`/`await` syntax.
- Handling **concurrent tasks** (`tokio::spawn`) and timers (`tokio::time`).
- Understanding **TCP/UDP sockets** in async Rust.

---

## **3. Networking Concepts**

- **TCP/UDP basics**: sending/receiving messages.
- Understanding **message serialization/deserialization** (`serde` recommended).
- Concepts of **peer-to-peer communication**.

---

## **4. Graph Algorithms**

- **Shortest path algorithms** (Dijkstra).
- **Overlay propagation algorithms** (gossip, MST) for distributing information.
- Representing networks as **graphs** (`petgraph` crate recommended).

---

## **5. Distributed Systems Concepts**

- **Decentralized decision-making** (nodes operate independently).
- **Load balancing** and resource sharing among nodes.
- Optional: handling **node joins/leaves** and **failure simulation**.

Plan

## 

## **Week 1: Core Setup and Basic Functionality**

### **Day 1–2: Rust & Project Setup**

- Set up a new Rust project with Cargo.
- Add necessary dependencies:

```toml
[dependencies]
tokio = { version ="1", features = ["full"] }
serde = { version ="1", features = ["derive"] }
petgraph ="0.7"
rand ="0.8"

```

- Define **basic structs**: `Node`, `Job`, `Message`.
- Implement a **network graph with leaf-spine topology** using `petgraph`:
    - **Leaf nodes** = servers
    - **Spine nodes** = switches connecting leaf nodes
    - Edges can have **weights** for latency/bandwidth to simulate realistic AI data center conditions.
---

### **Day 3–4: Graph Algorithms & Scheduling Logic**

- Implement **shortest path algorithm** (Dijkstra) for job forwarding across the leaf-spine topology.
- Implement **basic job scheduling**:
    - Assign jobs locally if resources are available.
    - Forward jobs to the nearest node with sufficient resources using shortest-path computation.
- Simulate **job queueing, resource usage, and network-aware routing** (take edge weights into account).

---

### **Day 5: Networking Basics**

- Implement **async TCP/UDP communication** with `tokio`.
- Define **message serialization/deserialization** using `serde`.
- Test **sending and receiving messages** between nodes in the leaf-spine topology.

---

## **Week 2: Distributed Simulation and Enhancements**

### **Day 6–7: P2P Communication**

- Connect multiple nodes in a **peer-to-peer network** (simulated as async tasks).
- Implement **job propagation** across the leaf-spine network:
    - Nodes share their **resource availability** with neighbors (leaf-to-leaf via spine nodes).
    - Forward jobs to nodes with available capacity using network-aware routing.
- Implement **periodic gossip messages** to update neighbors efficiently.

---

### **Day 8: Advanced Scheduling & Algorithms**

- Optional: implement **MST or optimized gossip overlays** across the leaf-spine topology for more efficient propagation.
- Handle **multiple concurrent jobs**, taking into account **network costs along the paths**.
- Optionally simulate **job priorities** and preemption.


Project structure 

```
ai-p2p-scheduler/
├── Cargo.toml
├── README.md
└── src/
    ├── main.rs
    │
    ├── config.rs
    ├── config/
    │   └── topology.rs
    │
    ├── graph.rs
    ├── graph/
    │   ├── datacenter.rs
    │   ├── shortest_path.rs
    │   └── mst.rs            # (optional, later)
    │
    ├── network.rs
    ├── network/
    │   ├── transport.rs
    │   ├── messages.rs
    │   └── gossip.rs
    │
    ├── scheduler.rs
    ├── scheduler/
    │   ├── job.rs
    │   └── scheduler.rs
    │
    ├── node.rs
    ├── node/
    │   └── node.rs
    │
    ├── routing.rs            # (add later)
    ├── routing/
    │   ├── ecmp.rs
    │   └── routing_table.rs
    │
    └── utils.rs
        └── metrics.rs
```