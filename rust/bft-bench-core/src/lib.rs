use std::collections::{HashMap, HashSet};
use std::time::{Duration, Instant};

use reader::ReaderReply;
use tokio::task::spawn;
use tokio::time::sleep;
use tokio::{sync::broadcast, sync::mpsc};
use uuid::Uuid;

use histogram::Histogram;
use worker::WorkerRequest;
use writer::WriterReply;

pub mod bft_binding;
pub mod config;
pub mod result;
pub mod stats;
pub use {bft_binding::*, config::*, result::*, stats::*};

mod reader;
mod worker;
mod writer;

const UUID_SIZE: usize = 16;

const CONTROL_CHANNELS_BUFFER: usize = 1;
const DATA_CHANNELS_BUFFER: usize = 1024 * 1024;

struct InProgressWrite {
    write_start: Instant,
    nodes_awaiting_read: HashSet<usize>,
}

struct BftBenchmarkState {
    in_progress_writes: HashMap<Uuid, Option<InProgressWrite>>,
    tx_writers_control: broadcast::Sender<WorkerRequest>,
    rx_incoming_writes: mpsc::Receiver<WriterReply>,
    tx_readers_control: broadcast::Sender<WorkerRequest>,
    rx_incoming_reads: mpsc::Receiver<ReaderReply>,
    stats: Stats,
}

pub async fn run<B: BftBinding + 'static>(config: Config, mut bft_binding: B) -> Result<Stats> {
    let (tx_writers_control, rx_writers_control) = broadcast::channel(CONTROL_CHANNELS_BUFFER);
    let (tx_incoming_writes, rx_incoming_writes) = mpsc::channel(DATA_CHANNELS_BUFFER);
    let (tx_readers_control, rx_readers_control) = broadcast::channel(CONTROL_CHANNELS_BUFFER);
    let (tx_incoming_reads, rx_incoming_reads) = mpsc::channel(DATA_CHANNELS_BUFFER);

    let mut state = BftBenchmarkState {
        in_progress_writes: HashMap::<Uuid, Option<InProgressWrite>>::new(),
        tx_writers_control,
        rx_incoming_writes,
        tx_readers_control,
        rx_incoming_reads,
        stats: Stats::new(
            config.nodes.len(),
            config
                .nodes
                .iter()
                .filter(|node| match **node {
                    Node::ReadWrite(_) => true,
                    _ => false,
                })
                .collect::<Vec<_>>()
                .len(),
        ),
    };

    let mut writers_to_go = config
        .nodes
        .iter()
        .enumerate()
        .map(|(node_idx, _)| node_idx)
        .collect::<HashSet<_>>();

    let mut accesses = HashMap::<usize, NodeAccess<B::Writer, B::Reader>>::new();
    for (node_idx, node) in config.nodes.into_iter().enumerate() {
        accesses.insert(node_idx, bft_binding.access(node).await);
    }

    let read_indices = start_reads::<B>(&accesses, rx_readers_control, tx_incoming_reads);
    let mut readers_to_go = read_indices.clone();

    log::info!("Read indices: {:?}", read_indices);

    start_writes::<B>(
        config.transaction_size - UUID_SIZE - 1,
        &accesses,
        config.write_interval,
        rx_writers_control,
        tx_incoming_writes,
    );

    log::info!(
        "The benchmark will be run for {} seconds",
        config.run_duration.as_secs_f64()
    );

    let (tx_complete, mut rx_complete) = mpsc::channel(CONTROL_CHANNELS_BUFFER);
    spawn(async move {
        sleep(config.run_duration).await;
        tx_complete
            .send(())
            .await
            .expect("Cannot send benchmark completion request");
    });

    loop {
        tokio::select! {
            write = state.rx_incoming_writes.recv() => {
                match write {
                    Some(write) => {
                        match write {
                            WriterReply::SuccessfulWrite { write_start, write_duration, uuid, node_idx } => {
                                log::info!("Writer {} succeeded write {}", node_idx, uuid);
                                let write_duration_nanos = u64_nanos(write_duration);
                                increment_histogram(&mut state.stats.global_writes_successful_nanos, write_duration_nanos);
                                increment_histogram(unwrap_histogram(
                                    state.stats.node_writes_successful_nanos.get_mut(node_idx),
                                ), write_duration_nanos);
                                state.in_progress_writes.insert(
                                    uuid,
                                    Some(InProgressWrite {
                                        write_start,
                                        nodes_awaiting_read: read_indices.clone(),
                                    })
                                );
                                log::info!(
                                    "In-progress writes count after Writer {} spawned write {}: {}",
                                    node_idx,
                                    uuid,
                                    state.in_progress_writes.len()
                                );
                            },
                            WriterReply::FailedWrite { write_duration, uuid, node_idx } => {
                                log::error!("Writer {} failed write {}", node_idx, uuid);
                                let write_duration_nanos = u64_nanos(write_duration);
                                increment_histogram(&mut state.stats.global_writes_failed_nanos, write_duration_nanos);
                                increment_histogram(unwrap_histogram(
                                    state.stats.node_writes_failed_nanos.get_mut(node_idx),
                                ), write_duration_nanos);
                            },
                            WriterReply::Completed { node_idx } => {
                                log::info!("Writer {} completed", node_idx);
                                writers_to_go.remove(&node_idx);
                                if writers_to_go.is_empty() && readers_to_go.is_empty() {
                                    break;
                                }
                            },
                        };
                    },
                    None => log::info!("All writers closed the sender"),
                }
            }
            read = state.rx_incoming_reads.recv() => {
                match read {
                    Some(read) => {
                        match read {
                            ReaderReply::SuccessfulRead { read_duration, uuid, node_idx } => {
                                let read_duration_nanos = u64_nanos(read_duration);
                                increment_histogram(&mut state.stats.global_reads_successful_nanos, read_duration_nanos);
                                increment_histogram(unwrap_histogram(
                                    state.stats.node_reads_successful_nanos.get_mut(node_idx),
                                ), read_duration_nanos);
                                match state.in_progress_writes.remove(&uuid) {
                                    Some(Some(InProgressWrite {
                                        write_start,
                                        mut nodes_awaiting_read,
                                    })) => {
                                        let node_round_trip_nanos = u64_nanos(write_start.elapsed());

                                        log::info!(
                                            "Reader {}: in-progress transaction {} completed in {} nanos",
                                            node_idx,
                                            uuid,
                                            node_round_trip_nanos
                                        );

                                        increment_histogram(
                                            unwrap_histogram(state.stats.nodes_roundtrip_nanos.get_mut(node_idx)),
                                            node_round_trip_nanos,
                                        );

                                        if nodes_awaiting_read.remove(&node_idx) {
                                            if nodes_awaiting_read.len() == 0 {
                                                // Last read
                                                log::info!(
                                                    "Reader {}: last read for transaction {}",
                                                    node_idx,
                                                    uuid
                                                );
                                                increment_histogram(
                                                    &mut state.stats.global_roundtrip_nanos,
                                                    node_round_trip_nanos,
                                                );
                                            } else {
                                                log::info!(
                                                    "Reader {}: {} reads still pending for transaction {}",
                                                    node_idx,
                                                    nodes_awaiting_read.len(),
                                                    uuid
                                                );
                                                state.in_progress_writes.insert(
                                                    uuid,
                                                    Some(InProgressWrite {
                                                        write_start,
                                                        nodes_awaiting_read,
                                                    }),
                                                );
                                            }
                                        } else {
                                            log::error!("Reader {}: duplicate read {}", node_idx, uuid);
                                            panic!("Reader {}: duplicate read {}", node_idx, uuid);
                                        }
                                    }
                                    _ => {
                                        log::error!("Reader {}: duplicate read {}", node_idx, uuid);
                                        panic!("Reader {}: duplicate read {}", node_idx, uuid)
                                    }
                                }
                                log::info!(
                                    "Reader {}: in-progress writes count after {} read completed: {}",
                                    node_idx,
                                    uuid,
                                    state.in_progress_writes.len()
                                );
                            },

                            ReaderReply::FailedRead { read_duration, bft_error, node_idx } => {
                                log::error!("Reader {} failed read: {}", node_idx, bft_error);
                                let read_duration_nanos = u64_nanos(read_duration);
                                increment_histogram(&mut state.stats.global_reads_failed_nanos, read_duration_nanos);
                                increment_histogram(unwrap_histogram(
                                    state.stats.node_reads_failed_nanos.get_mut(node_idx),
                                ), read_duration_nanos);
                            },
                            ReaderReply::Completed { node_idx } => {
                                log::info!("Reader {} completed", node_idx);
                                readers_to_go.remove(&node_idx);
                                if writers_to_go.is_empty() && readers_to_go.is_empty() {
                                    break;
                                }
                            },
                        };
                    },
                    None =>  log::info!("All readers closed the sender"),
                }
                ()
            }
            _ = rx_complete.recv() => {
                log::info!("Benchmark duration elapsed, requesting readers' and writers' completion");
                request_stop(&mut state);
            }
        }
    }

    Ok(state.stats)
}

fn start_writes<B: BftBinding + 'static>(
    value_size: usize,
    accesses: &HashMap<usize, NodeAccess<B::Writer, B::Reader>>,
    write_interval: Duration,
    rx_writers_control: broadcast::Receiver<WorkerRequest>,
    tx_incoming_writes: mpsc::Sender<WriterReply>,
) {
    log::info!("Starting writers");
    for (node_idx, writer) in accesses.iter().map(|(node_idx, access)| {
        (
            node_idx,
            match access {
                NodeAccess::ReadWriteAccess { writer, reader: _ } => writer,
                NodeAccess::WriteOnlyAccess { writer } => writer,
            },
        )
    }) {
        log::info!("Starting writer for node {}", node_idx);
        let writer = writer.clone();
        spawn(writer::write::<B::Writer>(
            value_size,
            writer,
            *node_idx,
            write_interval,
            rx_writers_control.resubscribe(),
            tx_incoming_writes.clone(),
        ));
    }
}

fn start_reads<B: BftBinding + 'static>(
    accesses: &HashMap<usize, NodeAccess<B::Writer, B::Reader>>,
    rx_readers_control: broadcast::Receiver<WorkerRequest>,
    tx_incoming_reads: mpsc::Sender<ReaderReply>,
) -> HashSet<usize> {
    log::info!("Starting readers");
    let mut read_indices = HashSet::<usize>::new();
    for (node_idx, access) in accesses.iter() {
        match access {
            NodeAccess::ReadWriteAccess { writer: _, reader } => {
                read_indices.insert(*node_idx);
                let reader = reader.clone();
                spawn(reader::read::<B::Reader>(
                    *node_idx,
                    reader,
                    rx_readers_control.resubscribe(),
                    tx_incoming_reads.clone(),
                ));
            }
            _ => (),
        }
        log::info!("start_reads: state unlocked");
    }
    read_indices
}

fn request_stop(state: &mut BftBenchmarkState) {
    log::info!("Signalling workers to stop");
    state
        .tx_writers_control
        .send(WorkerRequest::Stop())
        .expect("Cannot send writers completion request");
    state
        .tx_readers_control
        .send(WorkerRequest::Stop())
        .expect("Cannot send readers completion request");
}

fn unwrap_histogram(histo: Option<&mut Histogram>) -> &mut Histogram {
    histo.expect("Internal error: not enough histograms created")
}

fn increment_histogram(histo: &mut Histogram, elapsed_nanos: u64) {
    histo
        .increment(elapsed_nanos)
        .expect("Internal error: cannot increment histogram");
}

fn u64_nanos(duration: Duration) -> u64 {
    u64::try_from(duration.as_nanos()).expect("Internal error: duration nanos don't fit 64 bits")
}
