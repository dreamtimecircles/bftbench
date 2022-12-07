use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use std::{error::Error, time::Duration};

use async_trait::async_trait;

use bytes::Bytes;
use rand::RngCore;
use tokio::task::{spawn, JoinHandle};
use tokio::time;
use uuid::Uuid;

use serde_derive::{Deserialize, Serialize};

use metrics::histogram;

const UUID_SIZE: usize = 16;

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

#[async_trait]
pub trait BftBinding {
    type Writer: BftWriter;
    type Reader: BftReader;

    fn new() -> Self;
    async fn access(&mut self, node: Node) -> NodeAccess<Self::Writer, Self::Reader>;
}

pub enum NodeAccess<Writer: BftWriter, Reader: BftReader> {
    ReadWriteAccess { writer: Writer, reader: Reader },
    WriteOnlyAccess { writer: Writer },
}

#[async_trait]
pub trait BftWriter: Send + Clone {
    async fn write(&mut self, key: Uuid, value: Bytes) -> Result<()>;
}

#[async_trait]
pub trait BftReader: Send + Clone {
    async fn read(&mut self) -> Result<Uuid>;
}

pub async fn run<B: BftBinding + 'static>(config: Config, mut bft_binding: B) -> Result<()> {
    let value = create_value(&config);

    let writes_running_mutex_arc = Arc::new(Mutex::new(true));
    let reads_running_mutex_arc = Arc::new(Mutex::new(true));

    let in_progress_writes_mutex_arc =
        Arc::new(Mutex::new(HashMap::<Uuid, Option<InProgressWrite>>::new()));
    let in_progress_reads_mutex = Mutex::new(Vec::<Option<JoinHandle<()>>>::new());

    let mut accesses = HashMap::<usize, NodeAccess<B::Writer, B::Reader>>::new();
    for (node_idx, node) in config.nodes.into_iter().enumerate() {
        accesses.insert(node_idx, bft_binding.access(node).await);
    }

    let read_indices = start_reads::<B>(
        &accesses,
        &in_progress_reads_mutex,
        &in_progress_writes_mutex_arc,
        &reads_running_mutex_arc,
    );

    start_writes::<B>(
        value.clone(),
        &accesses,
        config.write_interval,
        &in_progress_writes_mutex_arc,
        read_indices,
        &writes_running_mutex_arc,
    );

    log::debug!("Waiting for {:?}", config.run_duration);

    let mut run_duration = time::interval(config.run_duration);
    run_duration.tick().await; // Immediate
    run_duration.tick().await;

    log::debug!("Stopping benchmark");

    // Stop reading and writing
    stop(
        in_progress_writes_mutex_arc,
        in_progress_reads_mutex,
        reads_running_mutex_arc,
        writes_running_mutex_arc,
    )
    .await?;

    Ok(())
}

struct InProgressWrite {
    start: Instant,
    join_handle: JoinHandle<Result<()>>,
    nodes_awaiting_read: HashSet<usize>,
}

fn create_value(config: &Config) -> Bytes {
    let value_size = config.transaction_size - UUID_SIZE - 1;
    let mut value = vec![0u8; value_size];
    rand::rngs::OsRng.fill_bytes(&mut value);
    log::debug!("Random value of size {} generated", value_size);
    Bytes::from(value)
}

fn start_writes<B: BftBinding + 'static>(
    value: Bytes,
    accesses: &HashMap<usize, NodeAccess<B::Writer, B::Reader>>,
    write_interval: Duration,
    in_progress_writes_mutex_arc: &Arc<Mutex<HashMap<Uuid, Option<InProgressWrite>>>>,
    read_indices: HashSet<usize>,
    running_mutex_arc: &Arc<Mutex<bool>>,
) {
    log::debug!("Starting writers");
    let read_indices_arc = Arc::new(read_indices);
    for (node_idx, writer) in accesses.iter().map(|(node_idx, access)| {
        (
            node_idx,
            match access {
                NodeAccess::ReadWriteAccess { writer, reader: _ } => writer,
                NodeAccess::WriteOnlyAccess { writer } => writer,
            },
        )
    }) {
        log::debug!("Starting writer for node {}", node_idx);
        let value = value.clone();
        let writer = writer.clone();
        spawn(write::<B::Writer>(
            value,
            writer,
            *node_idx,
            write_interval,
            in_progress_writes_mutex_arc.clone(),
            read_indices_arc.clone(),
            running_mutex_arc.clone(),
        ));
    }
}

async fn write<W: BftWriter + 'static>(
    value: Bytes,
    writer: W,
    node_idx: usize,
    write_interval: Duration,
    in_progress_writes_mutex_arc: Arc<Mutex<HashMap<Uuid, Option<InProgressWrite>>>>,
    read_indices_arc: Arc<HashSet<usize>>,
    running_mutex_arc: Arc<Mutex<bool>>,
) {
    let mut write_interval = time::interval(write_interval);
    write_interval.tick().await; // Immediate
    loop {
        {
            let running_locked = running_mutex_arc.lock().unwrap();
            let running = *running_locked;
            if !running {
                log::debug!(
                    "Shutting down, bailing out from writes for node {}",
                    node_idx
                );
                break;
            }
        }
        {
            let mut writes_locked = in_progress_writes_mutex_arc.lock().unwrap();
            let uuid = Uuid::new_v4();
            let value = value.clone();
            let mut writer = writer.clone();
            (*writes_locked).insert(
                uuid,
                Some(InProgressWrite {
                    start: Instant::now(),
                    join_handle: spawn(async move {
                        log::debug!("Starting write");
                        let write_start = Instant::now();
                        let result = writer.write(uuid, value).await;
                        let write_elapsed = write_start.elapsed();
                        let outcome = match result {
                            Ok(()) => {
                                log::debug!("Write successful");
                                "successful"
                            }
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
        log::debug!("Waiting for next schedule");
        write_interval.tick().await;
    }
}

fn start_reads<B: BftBinding + 'static>(
    accesses: &HashMap<usize, NodeAccess<B::Writer, B::Reader>>,
    in_progress_reads_mutex: &Mutex<Vec<Option<JoinHandle<()>>>>,
    in_progress_writes_mutex_arc: &Arc<Mutex<HashMap<Uuid, Option<InProgressWrite>>>>,
    running_mutex_arc: &Arc<Mutex<bool>>,
) -> HashSet<usize> {
    log::debug!("Starting readers");
    let mut read_indices = HashSet::<usize>::new();
    for (node_idx, access) in accesses.iter() {
        read_indices.insert(*node_idx);
        let mut reads = in_progress_reads_mutex.lock().unwrap();
        match access {
            NodeAccess::ReadWriteAccess { writer: _, reader } => {
                let reader = reader.clone();
                (*reads).push(Some(spawn(read::<B::Reader>(
                    *node_idx,
                    reader,
                    in_progress_writes_mutex_arc.clone(),
                    running_mutex_arc.clone(),
                ))));
            }
            _ => (),
        }
    }
    read_indices
}

async fn read<R: BftReader + 'static>(
    node_idx: usize,
    mut reader: R,
    in_progress_writes_mutex_arc: Arc<Mutex<HashMap<Uuid, Option<InProgressWrite>>>>,
    running_mutex_arc: Arc<Mutex<bool>>,
) {
    log::debug!("Starting reader for node {}", node_idx);
    loop {
        {
            let running_locked = running_mutex_arc.lock().unwrap();
            let running = *running_locked;
            if !running {
                log::debug!(
                    "Shutting down, bailing out from reads for node {}",
                    node_idx
                );
                break;
            }
        }

        let read_start = Instant::now();

        log::debug!("Reading");
        let read_result = reader.read().await;
        let read_elapsed = read_start.elapsed();
        let &mut outcome;

        match read_result {
            Ok(uuid) => {
                log::debug!("Read transaction {}", uuid);

                outcome = "successful";
                let mut writes_locked = in_progress_writes_mutex_arc.lock().unwrap();
                match (*writes_locked).remove(&uuid) {
                    Some(Some(InProgressWrite {
                        start,
                        join_handle,
                        mut nodes_awaiting_read,
                    })) => {
                        log::debug!("In-progress read found for transaction {}", uuid);

                        let node_round_trip = start.elapsed();
                        histogram!(format!("node{}.round_trip", node_idx), node_round_trip);

                        if nodes_awaiting_read.remove(&node_idx) {
                            if nodes_awaiting_read.len() == 0 {
                                // Last read
                                log::debug!("Last read for transaction {}", uuid);
                                histogram!("global.round_trip", node_round_trip);
                            } else {
                                log::debug!(
                                    "{} reads still pending for transaction {}",
                                    nodes_awaiting_read.len(),
                                    uuid
                                );
                                (*writes_locked).insert(
                                    uuid,
                                    Some(InProgressWrite {
                                        start,
                                        join_handle,
                                        nodes_awaiting_read,
                                    }),
                                );
                            }
                        } else {
                            panic!("Duplicate read {}", uuid);
                        }
                    }
                    _ => panic!("Duplicate read {}", uuid),
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
    in_progress_writes_mutex_arc: Arc<Mutex<HashMap<Uuid, Option<InProgressWrite>>>>,
    in_progress_reads_mutex: Mutex<Vec<Option<JoinHandle<()>>>>,
    reads_running_mutex_arc: Arc<Mutex<bool>>,
    writes_running_mutex_arc: Arc<Mutex<bool>>,
) -> Result<()> {
    log::debug!("Switching off reads");
    *reads_running_mutex_arc.lock().unwrap() = false;
    log::debug!("Waiting for readers to finish");
    let mut in_progress_reads_locked = in_progress_reads_mutex.lock().unwrap();
    for read_join_handle in in_progress_reads_locked.iter_mut() {
        read_join_handle
            .take()
            .expect("Read future already awaited for")
            .await
            .unwrap();
    }
    log::debug!("Switching off writes");
    *writes_running_mutex_arc.lock().unwrap() = false;
    let mut in_progress_writes_locked = in_progress_writes_mutex_arc.lock().unwrap();
    log::debug!("Waiting for writers to finish");
    for (_, write_join_handle) in in_progress_writes_locked.iter_mut() {
        write_join_handle
            .take()
            .expect("Write future already awaited for")
            .join_handle
            .await
            .unwrap()?;
    }
    Ok(())
}
