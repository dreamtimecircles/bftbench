use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt::Display;
use std::sync::{Arc, Mutex};
use std::{error::Error, time::Duration};

use async_trait::async_trait;

use rand::RngCore;
use tokio::task::{spawn, JoinHandle};
use tokio::time;
use uuid::Uuid;

use serde_derive::{Deserialize, Serialize};

#[derive(Debug)]
pub struct BftError {
    pub message: Cow<'static, str>,
}

impl Display for BftError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl Error for BftError {}

pub type Result<T> = core::result::Result<T, BftError>;

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    write_interval: Duration,
    transaction_size: usize,
    nodes: Vec<Node>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Node {
    pub endpoint: NodeEndpoint,
    pub kind: NodeKind,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct NodeEndpoint {
    pub host: String,
    pub port: u16,
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub enum NodeKind {
    Write,
    ReadWrite,
}

#[async_trait]
pub trait BftBinding {
    async fn write(
        write_endpoint: Arc<NodeEndpoint>,
        key: [u8; 16],
        value: Arc<Vec<u8>>,
    ) -> Result<()>;
    async fn read(read_endpoint: &NodeEndpoint) -> Result<[u8; 16]>;
}

pub async fn run<B: BftBinding>(config: Config) -> Result<()> {
    let mut value = vec![0u8; config.transaction_size - 26];
    rand::rngs::OsRng.fill_bytes(&mut value);

    let in_progress_writes_arc = Arc::new(Mutex::new(
        HashMap::<[u8; 16], JoinHandle<Result<()>>>::new(),
    ));
    let in_progress_reads = Mutex::new(Vec::<JoinHandle<Result<()>>>::new());

    let mut running = true;

    // Setup read threads
    let nodes_arcs: Vec<_> = config.nodes.into_iter().map(|n| Arc::new(n)).collect();
    let nodes_write_endpoint_arcs: Vec<_> = nodes_arcs
        .iter()
        .map(|arc| Arc::new(arc.endpoint.clone()))
        .collect();
    let nodes_read_arcs: Vec<_> = nodes_arcs.iter().map(|arc| arc.clone()).collect();

    for n in nodes_read_arcs {
        if n.kind == NodeKind::ReadWrite {
            let mut reads = in_progress_reads.lock().unwrap();
            let in_progress_writes = in_progress_writes_arc.clone();
            (*reads).push(spawn(async move {
                while running {
                    let uuid = B::read(&n.endpoint).await?;
                    let mut writes = in_progress_writes.lock().unwrap();
                    match (*writes).remove(&uuid) {
                        Some(handle) => {
                            if !handle.is_finished() {
                                return Err(BftError {
                                    message: Cow::Borrowed("Write still pending"),
                                });
                            }
                        }
                        None => {
                            return Err(BftError {
                                message: Cow::Borrowed("Duplicate read"),
                            })
                        }
                    }
                }
                Ok(())
            }));
        }
    }

    // Start writes
    let value_arc = Arc::new(value);
    for n in nodes_write_endpoint_arcs {
        let in_progress_writes = in_progress_writes_arc.clone();
        let value_arc = value_arc.clone();
        spawn(async move {
            while running {
                let uuid = Uuid::new_v4().into_bytes();
                let write_join_handle = spawn(B::write(n.clone(), uuid, value_arc.clone()));
                {
                    let mut writes = in_progress_writes.lock().unwrap();
                    (*writes).insert(uuid, write_join_handle);
                }
                time::interval(config.write_interval).tick().await;
            }
        });
    }

    Ok(())
}
