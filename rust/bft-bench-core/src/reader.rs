use std::time::{Duration, Instant};

use tokio::sync::{broadcast, mpsc};
use uuid::Uuid;

use crate::{worker::WorkerRequest, BftError, BftReader};

#[derive(Debug)]
pub(crate) enum ReaderReply {
    SuccessfulRead {
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
    log::info!("Starting reader for node {}", node_idx);
    loop {
        if let Ok(WorkerRequest::Stop()) = rx_readers_control.try_recv() {
            log::info!("Reader {}: ending", node_idx);
            tx_incoming_reads
                .send(ReaderReply::Completed { node_idx })
                .await
                .expect("Reader completion message couldn't be sent");
            break;
        }

        let read_start = Instant::now();

        log::info!("Reader {}: reading", node_idx);
        let read_result = reader.read().await;
        let read_duration = read_start.elapsed();
        match read_result {
            Ok(uuid) => {
                log::info!("Reader {}: read transaction {}", node_idx, uuid);
                tx_incoming_reads
                    .send(ReaderReply::SuccessfulRead {
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
}
