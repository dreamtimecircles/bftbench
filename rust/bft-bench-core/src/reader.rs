use std::time::{Duration, Instant};

use tokio::sync::{broadcast, mpsc};
use uuid::Uuid;

use crate::{worker::WorkerRequest, BftError, BftReader};

#[derive(Debug)]
pub(crate) enum ReaderReply {
    SuccessfulRead {
        read_completion_instant: Instant,
        read_duration: Duration,
        uuid: Uuid,
        node_idx: usize,
    },
    FailedRead {
        read_duration: Duration,
        bft_error: BftError,
        node_idx: usize,
    },
    Completed {
        node_idx: usize,
    },
}

pub(crate) async fn read<R: BftReader + 'static>(
    node_idx: usize,
    mut reader: R,
    mut rx_readers_control: broadcast::Receiver<WorkerRequest>,
    tx_incoming_reads: mpsc::Sender<ReaderReply>,
) {
    loop {
        let read_start = Instant::now();
        log::debug!("Reader {}: reading", node_idx);
        match tokio::time::timeout(Duration::from_secs(60), reader.read()).await {
            Ok(read_result) => {
                let read_completion_instant = Instant::now();
                let read_duration = read_start.elapsed();
                match read_result {
                    Ok(uuid) => {
                        log::debug!("Reader {}: read transaction {}", node_idx, uuid);
                        tx_incoming_reads
                            .send(ReaderReply::SuccessfulRead {
                                read_completion_instant,
                                read_duration,
                                uuid,
                                node_idx,
                            })
                            .await
                            .expect("Receiver closed");
                    }
                    Err(bft_error) => {
                        log::error!("Reader {}: read failed: {}", node_idx, bft_error);
                        tx_incoming_reads
                            .send(ReaderReply::FailedRead {
                                read_duration,
                                node_idx,
                                bft_error,
                            })
                            .await
                            .expect("Receiver closed");
                    }
                }
            }
            Err(_) => {
                if let Ok(WorkerRequest::Stop()) = rx_readers_control.try_recv() {
                    log::debug!("Reader {}: ending", node_idx);
                    tx_incoming_reads
                        .send(ReaderReply::Completed { node_idx })
                        .await
                        .expect("Reader completion message couldn't be sent");
                    break;
                } else {
                    log::warn!("Reader {}: timeout", node_idx);
                }
            }
        }
    }
}
