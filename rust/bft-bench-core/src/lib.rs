//! A benchmarking framework geared towards BFT ordering libraries and platforms.

use std::collections::{HashMap, HashSet};
use std::time::{Duration, Instant};

use histogram::Histogram;
use tokio::task::spawn;
use tokio::time::sleep;
use tokio::{sync::broadcast, sync::mpsc};
use uuid::Uuid;

use reader::ReaderReply;
use stats::*;
use worker::WorkerRequest;
use writer::WriterReply;
pub use {bft_binding::*, config::*, result::*};

pub mod bft_binding;
pub mod config;
pub mod result;
pub mod stats;

mod reader;
mod worker;
mod writer;

pub const UUID_SIZE: usize = 16;

const CONTROL_CHANNELS_BUFFER: usize = 1;
const DATA_CHANNELS_BUFFER: usize = 128 * 1024 * 1024;

enum WriteStatus {
    Written {
        write_start: Instant,
        nodes_awaiting_read: HashSet<usize>,
    },
    ReadWhenWriteDataAvailable {
        read_completion_instant: Instant,
        node_idx: usize,
    },
}

struct BftBenchmarkState {
    in_progress_writes: HashMap<Uuid, WriteStatus>,
    tx_writers_control: broadcast::Sender<WorkerRequest>,
    rx_incoming_writes: mpsc::Receiver<WriterReply>,
    tx_readers_control: broadcast::Sender<WorkerRequest>,
    rx_incoming_reads: mpsc::Receiver<ReaderReply>,
    stats: Stats,
}

/// Runs a benchmark using the given [`BftBinding`] and [`Config`] and produces [`Stats`].
pub async fn run<B: BftBinding + 'static>(config: Config, mut bft_binding: B) -> Result<Reports> {
    let (tx_writers_control, rx_writers_control) = broadcast::channel(CONTROL_CHANNELS_BUFFER);
    let (tx_incoming_writes, rx_incoming_writes) = mpsc::channel(DATA_CHANNELS_BUFFER);
    let (tx_readers_control, rx_readers_control) = broadcast::channel(CONTROL_CHANNELS_BUFFER);
    let (tx_incoming_reads, rx_incoming_reads) = mpsc::channel(DATA_CHANNELS_BUFFER);

    let mut state = BftBenchmarkState {
        in_progress_writes: HashMap::<Uuid, WriteStatus>::new(),
        tx_writers_control,
        rx_incoming_writes,
        tx_readers_control,
        rx_incoming_reads,
        stats: Stats::new(
            Instant::now(),
            config.nodes.len(),
            config
                .nodes
                .iter()
                .filter(|node| matches!(**node, Node::ReadWrite(_)))
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

    let mut write_accesses = HashMap::<usize, B::Writer>::new();
    let mut read_accesses = HashMap::<usize, B::Reader>::new();
    for (node_idx, node) in config.nodes.into_iter().enumerate() {
        match bft_binding.access(node).await {
            NodeAccess::ReadWriteAccess { reader, writer } => {
                write_accesses.insert(node_idx, writer);
                read_accesses.insert(node_idx, reader);
            }
            NodeAccess::WriteOnlyAccess { writer } => {
                write_accesses.insert(node_idx, writer);
            }
        }
    }

    let read_indices = start_reads::<B>(
        read_accesses,
        rx_readers_control,
        tx_incoming_reads,
        config.read_grace,
    );
    let mut readers_to_go = read_indices.clone();

    log::info!("Read nodes: {:?}", read_indices);

    start_writes::<B>(
        write_accesses,
        config.write_interval,
        rx_writers_control,
        tx_incoming_writes,
    );

    let (tx_report, mut rx_report) = mpsc::channel(CONTROL_CHANNELS_BUFFER);
    let tx_report_periodic = tx_report.clone();
    spawn(async move {
        sleep(config.run_duration).await;
        tx_report
            .send(true)
            .await
            .expect("Cannot send benchmark completion request");
    });
    let _ = config.report_interval.map_or((), |i| {
        spawn(async move {
            loop {
                sleep(i).await;
                if tx_report_periodic.send(false).await.is_err() {
                    log::debug!("Report channel closed, stopping periodic report");
                    break;
                }
            }
        });
    });

    loop {
        tokio::select! {
            Some(write) = state.rx_incoming_writes.recv() => {
                if handle_writer_reply(&mut state, &mut writers_to_go, &mut readers_to_go, &read_indices, write) {
                    break;
                }
            }
            Some(read) = state.rx_incoming_reads.recv() => {
                if handle_reader_reply(&mut state, &mut writers_to_go, &mut readers_to_go, read) {
                    break;
                }
            }
            Some(complete) = rx_report.recv() => {
                if complete {
                    log::info!("Benchmark duration elapsed, requesting readers' and writers' completion");
                    rx_report.close();
                    request_stop(&mut state);
                } else {
                    log::info!("Periodic stats follow: {}", Into::<Reports>::into(&state.stats));
                }
            }
        }
    }

    Ok((&state.stats).into())
}

fn start_writes<B: BftBinding + 'static>(
    write_accesses: HashMap<usize, B::Writer>,
    write_interval: Duration,
    rx_writers_control: broadcast::Receiver<WorkerRequest>,
    tx_incoming_writes: mpsc::Sender<WriterReply>,
) {
    log::info!("Starting writers");
    for (node_idx, writer) in write_accesses.into_iter() {
        log::info!("Starting writer for node {}", node_idx);
        spawn(writer::write::<B::Writer>(
            writer,
            node_idx,
            write_interval,
            rx_writers_control.resubscribe(),
            tx_incoming_writes.clone(),
        ));
    }
}

fn start_reads<B: BftBinding + 'static>(
    read_accesses: HashMap<usize, B::Reader>,
    rx_readers_control: broadcast::Receiver<WorkerRequest>,
    tx_incoming_reads: mpsc::Sender<ReaderReply>,
    read_grace: Duration,
) -> HashSet<usize> {
    log::info!("Starting readers");
    let mut read_indices = HashSet::<usize>::new();
    for (node_idx, reader) in read_accesses.into_iter() {
        log::info!("Starting reader for node {}", node_idx);
        read_indices.insert(node_idx);
        spawn(reader::read::<B::Reader>(
            node_idx,
            reader,
            rx_readers_control.resubscribe(),
            tx_incoming_reads.clone(),
            read_grace,
        ));
    }
    read_indices
}

fn handle_writer_reply(
    state: &mut BftBenchmarkState,
    writers_to_go: &mut HashSet<usize>,
    readers_to_go: &mut HashSet<usize>,
    read_indices: &HashSet<usize>,
    write: WriterReply,
) -> bool {
    match write {
        WriterReply::SuccessfulWrite {
            write_start,
            write_duration,
            uuid,
            node_idx,
        } => {
            log::debug!("Writer {} succeeded write {}", node_idx, uuid);
            let write_duration_nanos = u64_nanos(write_duration);
            update_op_stat(
                &mut state.stats.global_write,
                state.stats.nodes_writes.get_mut(node_idx).unwrap(),
                write_duration_nanos,
                true,
            );
            match state.in_progress_writes.remove(&uuid) {
                Some(WriteStatus::ReadWhenWriteDataAvailable {
                    read_completion_instant,
                    node_idx,
                }) => {
                    let nodes_awaiting_read = read_indices.clone();
                    if !complete_read(
                        "Writer",
                        state,
                        nodes_awaiting_read,
                        &uuid,
                        node_idx,
                        &write_start,
                        read_completion_instant - write_start,
                    ) {
                        log::error!(
                            "Writer {} attempted read from non-read node {}",
                            node_idx,
                            uuid
                        );
                        panic!(
                            "Writer {} attempted read from non-read node {}",
                            node_idx, uuid
                        );
                    }
                }
                Some(WriteStatus::Written { .. }) => {
                    log::error!(
                        "Writer {} trying to perform a duplicate write {}",
                        node_idx,
                        uuid
                    );
                    panic!(
                        "Writer {} trying to perform a duplicate write {}",
                        node_idx, uuid
                    );
                }
                None => {
                    state.in_progress_writes.insert(
                        uuid,
                        WriteStatus::Written {
                            write_start,
                            nodes_awaiting_read: read_indices.clone(),
                        },
                    );
                }
            }
        }
        WriterReply::FailedWrite {
            write_duration,
            uuid,
            node_idx,
        } => {
            log::error!("Writer {} failed write {}", node_idx, uuid);
            let write_duration_nanos = u64_nanos(write_duration);
            update_op_stat(
                &mut state.stats.global_write,
                state.stats.nodes_writes.get_mut(node_idx).unwrap(),
                write_duration_nanos,
                false,
            );
        }
        WriterReply::Completed { node_idx } => {
            log::info!("Writer {} completed", node_idx);
            writers_to_go.remove(&node_idx);
            log::info!("Writers to go: {:?}", writers_to_go);
            log::info!("Readers to go: {:?}", readers_to_go);
            if writers_to_go.is_empty() && readers_to_go.is_empty() {
                return true;
            }
        }
    }

    false
}

fn handle_reader_reply(
    state: &mut BftBenchmarkState,
    writers_to_go: &mut HashSet<usize>,
    readers_to_go: &mut HashSet<usize>,
    read: ReaderReply,
) -> bool {
    match read {
        ReaderReply::SuccessfulRead {
            read_completion_instant,
            read_duration,
            uuid,
            node_idx,
        } => {
            log::debug!("Reader {} succeeded read {}", node_idx, uuid);
            let read_duration_nanos = u64_nanos(read_duration);
            update_op_stat(
                &mut state.stats.global_read.op,
                &mut state.stats.nodes_reads.get_mut(node_idx).unwrap().op,
                read_duration_nanos,
                true,
            );
            match state.in_progress_writes.remove(&uuid) {
                Some(WriteStatus::Written {
                    write_start,
                    nodes_awaiting_read,
                }) => {
                    if !complete_read(
                        "Reader",
                        state,
                        nodes_awaiting_read,
                        &uuid,
                        node_idx,
                        &write_start,
                        write_start.elapsed(),
                    ) {
                        log::error!("Duplicate read {} from reader {}: ", uuid, node_idx);
                        panic!("Duplicate read {} from reader {}: ", uuid, node_idx);
                    }
                }

                Some(WriteStatus::ReadWhenWriteDataAvailable { .. }) => {
                    log::error!("Duplicate read {} from reader {}: ", uuid, node_idx);
                    panic!("Duplicate read {} from reader {}: ", uuid, node_idx);
                }

                None => {
                    log::debug!(
                        "Reader {} found that write data for {} is not yet available",
                        node_idx,
                        uuid
                    );
                    state.in_progress_writes.insert(
                        uuid,
                        WriteStatus::ReadWhenWriteDataAvailable {
                            read_completion_instant,
                            node_idx,
                        },
                    );
                }
            }
            log::debug!(
                "In-progress writes count after Reader {} read {}: {}",
                node_idx,
                uuid,
                state.in_progress_writes.len()
            );
        }

        ReaderReply::FailedRead {
            read_duration,
            bft_error,
            node_idx,
        } => {
            log::error!("Reader {} failed read, error: {}", node_idx, bft_error);
            let read_duration_nanos = u64_nanos(read_duration);
            update_op_stat(
                &mut state.stats.global_read.op,
                &mut state.stats.nodes_reads.get_mut(node_idx).unwrap().op,
                read_duration_nanos,
                false,
            );
        }

        ReaderReply::Completed { node_idx } => {
            log::info!("Reader {} completed", node_idx);
            readers_to_go.remove(&node_idx);
            if writers_to_go.is_empty() && readers_to_go.is_empty() {
                return true;
            }
        }
    }

    false
}

fn complete_read(
    role: &'static str,
    state: &mut BftBenchmarkState,
    mut nodes_awaiting_read: HashSet<usize>,
    uuid: &Uuid,
    node_idx: usize,
    write_start: &Instant,
    node_round_trip: Duration,
) -> bool {
    let node_round_trip_nanos = u64_nanos(node_round_trip);

    log::debug!(
        "{} {} completed in-progress transaction {} in {} nanos",
        role,
        node_idx,
        uuid,
        node_round_trip_nanos
    );

    if nodes_awaiting_read.remove(&node_idx) {
        let now = Instant::now();
        update_stat(
            &mut state
                .stats
                .nodes_reads
                .get_mut(node_idx)
                .unwrap()
                .round_trip,
            now,
            node_round_trip_nanos,
        );
        if nodes_awaiting_read.is_empty() {
            // Last read
            log::debug!(
                "{} {} performed last read for transaction {}",
                role,
                node_idx,
                uuid
            );
            update_stat(
                &mut state.stats.global_read.round_trip,
                now,
                node_round_trip_nanos,
            );
        } else {
            log::debug!(
                "After read from {} {}, {} reads still pending for transaction {}",
                role,
                node_idx,
                nodes_awaiting_read.len(),
                uuid
            );
            state.in_progress_writes.insert(
                *uuid,
                WriteStatus::Written {
                    write_start: *write_start,
                    nodes_awaiting_read,
                },
            );
        };
        true
    } else {
        false
    }
}

fn request_stop(state: &mut BftBenchmarkState) {
    log::info!("Signalling workers to stop");
    state
        .tx_writers_control
        .send(WorkerRequest::Stop())
        .expect("Internal error: cannot send writers completion request");
    state
        .tx_readers_control
        .send(WorkerRequest::Stop())
        .expect("Internal error: cannot send readers completion request");
}

fn update_op_stat(
    global_op_stat: &mut OpStat,
    node_op_stat: &mut OpStat,
    duration_nanos: u64,
    ok: bool,
) {
    let now = Instant::now();
    if ok {
        update_stat(&mut global_op_stat.successful, now, duration_nanos);
        update_stat(&mut node_op_stat.successful, now, duration_nanos);
    } else {
        update_stat(&mut global_op_stat.failed, now, duration_nanos);
        update_stat(&mut node_op_stat.failed, now, duration_nanos);
    }
}

fn update_stat(stat: &mut Stat, now: Instant, duration_nanos: u64) {
    increment_histogram(&mut stat.histogram, duration_nanos / 1000);
    stat.counter.count += 1;
    stat.counter.now = now;
}

fn increment_histogram(histo: &mut Histogram, elapsed_micros: u64) {
    match histo.increment(elapsed_micros) {
        Ok(_) => {}
        Err(_) => log::error!(
            "Internal error: cannot increment histogram for {} micros",
            elapsed_micros
        ),
    }
}

fn u64_nanos(duration: Duration) -> u64 {
    u64::try_from(duration.as_nanos()).expect("Internal error: duration nanos don't fit 64 bits")
}
