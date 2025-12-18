use crate::node::node::Node;
use crate::scheduler::job::Job;
use chrono::{DateTime, Utc};


pub enum MessageType{
  JobRequest,
  JobForward,
  ResourceUpdate,
  Gossip,
}

pub struct Message {
  sender: Node,
  receiver: Node,
  payload: Job,
  message_type: MessageType,
  time_stamp: DateTime<Utc>
}