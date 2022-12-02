use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use std::{error::Error, time::Duration};

use async_trait::async_trait;

use rand::RngCore;
use tokio::task::{spawn, JoinHandle};
use tokio::time;
use uuid::Uuid;

use serde_derive::{Deserialize, Serialize};

use metrics::histogram;

#[derive(Debug)]
pub struct BftError {
    message: Cow<'static, str>,
}

impl Error for BftError {}

impl BftError {
    pub fn fixed(message: &'static str) -> Self {
        BftError {
            message: Cow::Borrowed(message),
        }
    }

    pub fn dynamic(message: String) -> Self {
        BftError {
            message: Cow::Owned(message),
        }
    }
}

impl Display for BftError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

pub type Result<T> = core::result::Result<T, BftError>;

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub run_duration: Duration,
    pub write_interval: Duration,
    pub transaction_size: usize,
    pub nodes: Vec<Node>,
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
pub trait BftWriter: Send + Sync {
    async fn write(
        &mut self,
        write_endpoint: Arc<NodeEndpoint>,
        key: [u8; 16],
        value: Arc<Vec<u8>>,
    ) -> Result<()>;
}

#[async_trait]
pub trait BftReader: Send + Sync {
    async fn read(&mut self, read_endpoint: &NodeEndpoint) -> Result<[u8; 16]>;
}

pub async fn run<W: BftWriter + 'static, R: BftReader + 'static>(
    config: Config,
    bft_writer: W,
    bft_reader: R,
) -> Result<()> {
    let value = create_value(&config);

    let in_progress_writes_mutex_arc = Arc::new(Mutex::new(HashMap::<
        [u8; 16],
        Option<InProgressWrite>,
    >::new()));
    let in_progress_reads_mutex = Mutex::new(Vec::<Option<JoinHandle<()>>>::new());

    let running_mutex_arc = Arc::new(Mutex::new(true));

    let nodes_arcs = config
        .nodes
        .into_iter()
        .map(|n| Arc::new(n))
        .collect::<Vec<_>>();

    let bft_reader_mutex_arc = Arc::new(tokio::sync::Mutex::new(bft_reader));

    let read_indices = start_reads::<R>(
        &bft_reader_mutex_arc,
        &nodes_arcs,
        &in_progress_reads_mutex,
        &in_progress_writes_mutex_arc,
        &running_mutex_arc,
    );

    let bft_writer_mutex_arc = Arc::new(tokio::sync::Mutex::new(bft_writer));

    start_writes::<W>(
        bft_writer_mutex_arc,
        Arc::new(value),
        config.write_interval,
        &in_progress_writes_mutex_arc,
        nodes_arcs,
        read_indices,
        &running_mutex_arc,
    );

    time::interval(config.run_duration).tick().await;

    // Stop reading and writing
    stop(
        in_progress_writes_mutex_arc,
        in_progress_reads_mutex,
        running_mutex_arc,
    )
    .await?;

    Ok(())
}

struct InProgressWrite {
    start: Instant,
    join_handle: JoinHandle<Result<()>>,
    nodes_awaiting_read: HashSet<usize>,
}

fn create_value(config: &Config) -> Vec<u8> {
    let mut value = vec![0u8; config.transaction_size - 26];
    rand::rngs::OsRng.fill_bytes(&mut value);
    value
}

fn start_writes<W: BftWriter + 'static>(
    bft_writer_mutex_arc: Arc<tokio::sync::Mutex<W>>,
    value_arc: Arc<Vec<u8>>,
    write_interval: Duration,
    in_progress_writes_mutex_arc: &Arc<Mutex<HashMap<[u8; 16], Option<InProgressWrite>>>>,
    nodes_arcs: Vec<Arc<Node>>,
    read_indices: HashSet<usize>,
    running_mutex_arc: &Arc<Mutex<bool>>,
) {
    let nodes_write_endpoint_arcs: Vec<_> = nodes_arcs
        .iter()
        .map(|arc| Arc::new(arc.endpoint.clone()))
        .collect();
    let read_indices_arc = Arc::new(read_indices);
    for (node_idx, node_endpoint_arc) in nodes_write_endpoint_arcs.into_iter().enumerate() {
        let value_arc = value_arc.clone();
        let bft_writer_mutex_arc = bft_writer_mutex_arc.clone();
        spawn(write::<W>(
            bft_writer_mutex_arc,
            value_arc,
            node_idx,
            node_endpoint_arc,
            write_interval,
            in_progress_writes_mutex_arc.clone(),
            read_indices_arc.clone(),
            running_mutex_arc.clone(),
        ));
    }
}

async fn write<W: BftWriter + 'static>(
    bft_writer_mutex_arc: Arc<tokio::sync::Mutex<W>>,
    value_arc: Arc<Vec<u8>>,
    node_idx: usize,
    node_endpoint_arc: Arc<NodeEndpoint>,
    write_interval: Duration,
    in_progress_writes_mutex_arc: Arc<Mutex<HashMap<[u8; 16], Option<InProgressWrite>>>>,
    read_indices_arc: Arc<HashSet<usize>>,
    running_mutex_arc: Arc<Mutex<bool>>,
) {
    loop {
        {
            let running_locked = running_mutex_arc.lock().unwrap();
            if !*running_locked {
                break;
            }
        }
        {
            let mut writes_locked = in_progress_writes_mutex_arc.lock().unwrap();
            let uuid = Uuid::new_v4().into_bytes();
            let node_endpoint_arc = node_endpoint_arc.clone();
            let value_arc = value_arc.clone();
            let bft_writer_mutex_arc = bft_writer_mutex_arc.clone();
            (*writes_locked).insert(
                uuid,
                Some(InProgressWrite {
                    start: Instant::now(),
                    join_handle: spawn(async move {
                        let write_start = Instant::now();
                        let result = bft_writer_mutex_arc
                            .lock()
                            .await
                            .write(node_endpoint_arc, uuid, value_arc)
                            .await;
                        let write_elapsed = write_start.elapsed();
                        let outcome = match result {
                            Ok(()) => "successful",
                            Err(ref bft_error) => {
                                log::error!("Write failed: {}", bft_error);
                                "failed"
                            }
                        };
                        histogram!(format!("global.write.{}", outcome), write_elapsed);
                        histogram!(format!("node{}.write.{}", node_idx, outcome), write_elapsed);
                        result
                    }),
                    nodes_awaiting_read: (*read_indices_arc).clone(),
                }),
            );
        }
        time::interval(write_interval).tick().await;
    }
}

fn start_reads<R: BftReader + 'static>(
    bft_reader_mutex_arc: &Arc<tokio::sync::Mutex<R>>,
    nodes_arcs: &Vec<Arc<Node>>,
    in_progress_reads_mutex: &Mutex<Vec<Option<JoinHandle<()>>>>,
    in_progress_writes_mutex_arc: &Arc<Mutex<HashMap<[u8; 16], Option<InProgressWrite>>>>,
    running_mutex_arc: &Arc<Mutex<bool>>,
) -> HashSet<usize> {
    let mut read_indices = HashSet::<usize>::new();
    for (node_idx, node) in nodes_arcs
        .iter()
        .map(|arc| arc.clone())
        .filter(|node| node.kind == NodeKind::ReadWrite)
        .enumerate()
    {
        read_indices.insert(node_idx);
        let mut reads = in_progress_reads_mutex.lock().unwrap();
        (*reads).push(Some(spawn(read::<R>(
            bft_reader_mutex_arc.clone(),
            node,
            node_idx,
            in_progress_writes_mutex_arc.clone(),
            running_mutex_arc.clone(),
        ))));
    }
    read_indices
}

async fn read<R: BftReader>(
    bft_reader_mutex_arc: Arc<tokio::sync::Mutex<R>>,
    node: Arc<Node>,
    node_idx: usize,
    in_progress_writes_mutex_arc: Arc<Mutex<HashMap<[u8; 16], Option<InProgressWrite>>>>,
    running_mutex_arc: Arc<Mutex<bool>>,
) {
    loop {
        {
            let running_locked = running_mutex_arc.lock().unwrap();
            if !*running_locked {
                break;
            }
        }

        let read_start = Instant::now();

        let read_result = bft_reader_mutex_arc.lock().await.read(&node.endpoint).await;
        let read_elapsed = read_start.elapsed();
        let &mut outcome;

        match read_result {
            Ok(uuid) => {
                outcome = "successful";
                let mut writes_locked = in_progress_writes_mutex_arc.lock().unwrap();
                match (*writes_locked).remove(&uuid) {
                    Some(Some(InProgressWrite {
                        start,
                        join_handle,
                        mut nodes_awaiting_read,
                    })) => {
                        if !join_handle.is_finished() {
                            panic!(
                                "Read transaction {:?} while its write is still pending",
                                uuid
                            );
                        }

                        let node_round_trip = start.elapsed();
                        histogram!(format!("node{}.round_trip", node_idx), node_round_trip);
                        if nodes_awaiting_read.len() == 1 {
                            // Last read
                            histogram!("global.round_trip", node_round_trip);
                        }

                        if nodes_awaiting_read.remove(&node_idx) {
                            (*writes_locked).insert(
                                uuid,
                                Some(InProgressWrite {
                                    start,
                                    join_handle,
                                    nodes_awaiting_read,
                                }),
                            );
                        } else {
                            panic!("Duplicate read {:?}", uuid);
                        }
                    }
                    _ => panic!("Duplicate read {:?}", uuid),
                }
            }
            Err(bft_error) => {
                log::error!("Read failed: {}", bft_error);
                outcome = "failed";
            }
        }

        histogram!(format!("global.read.{}", outcome), read_elapsed);
        histogram!(format!("node{}.read.{}", node_idx, outcome), read_elapsed);
    }
}

async fn stop(
    in_progress_writes_mutex_arc: Arc<Mutex<HashMap<[u8; 16], Option<InProgressWrite>>>>,
    in_progress_reads_mutex: Mutex<Vec<Option<JoinHandle<()>>>>,
    running_mutex_arc: Arc<Mutex<bool>>,
) -> Result<()> {
    *running_mutex_arc.lock().unwrap() = false;
    let mut in_progress_writes_locked = in_progress_writes_mutex_arc.lock().unwrap();
    for (_, write_join_handle) in in_progress_writes_locked.iter_mut() {
        write_join_handle
            .take()
            .expect("Write future already awaited for")
            .join_handle
            .await
            .unwrap()?;
    }
    let mut in_progress_reads_locked = in_progress_reads_mutex.lock().unwrap();
    for read_join_handle in in_progress_reads_locked.iter_mut() {
        read_join_handle
            .take()
            .expect("Read future already awaited for")
            .await
            .unwrap();
    }
    Ok(())
}
