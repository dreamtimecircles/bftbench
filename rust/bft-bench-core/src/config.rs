use std::time::Duration;

use serde_derive::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Config {
    pub run_duration: Duration,
    pub write_interval: Duration,
    pub transaction_size: usize,
    pub nodes: Vec<Node>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Node {
    Write(WriteNode),
    ReadWrite(ReadWriteNode),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReadWriteNode {
    pub node: WriteNode,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WriteNode {
    pub endpoint: NodeEndpoint,
}

#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct NodeEndpoint {
    pub host: String,
    pub port: u16,
}
