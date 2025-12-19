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
  id: String,
  sender: Node,
  receiver: Node,
  payload: Job,
  message_type: MessageType,
  time_stamp: DateTime<Utc>
}

impl Message {
  pub fn new(id: String, sender: Node, receiver: Node, payload: Job, message_type: MessageType) -> Self {
    Self {
      id,
      sender,
      receiver,
      payload,
      message_type,
      time_stamp: Utc::now()
    }
  }

  pub fn id(&self) -> &str{
    &self.id
  }

  pub fn sender(&self) -> &Node {
    &self.sender
  }

  pub fn receiver(&self) -> &Node {
    &self.receiver
  }

  pub fn payload(&self) -> &Job {
    &self.payload
  }

  pub fn message_type(&self) -> &MessageType {
    &self.message_type
  }

  pub fn time_stamp(&self) -> &DateTime<Utc> {
    &self.time_stamp
  }

}