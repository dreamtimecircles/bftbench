use std::time::{Duration, Instant};

use tokio::{
    spawn,
    sync::{broadcast, mpsc},
};
use uuid::Uuid;

use crate::{BftWriter, worker::WorkerRequest};

#[derive(Debug)]
pub(crate) enum WriterReply {
    SuccessfulWrite {
        write_start: Instant,
        write_duration: Duration,
        uuid: Uuid,
        node_idx: usize,
    },
    FailedWrite {
        write_duration: Duration,
        uuid: Uuid,
        node_idx: usize,
    },
    Completed {
        node_idx: usize,
    },
}

pub(crate) async fn write<W: BftWriter + 'static>(
    writer: W,
    node_idx: usize,
    write_interval: Duration,
    mut rx_writers_control: broadcast::Receiver<WorkerRequest>,
    tx_incoming_writes: mpsc::Sender<WriterReply>,
) {
    let mut interval = tokio::time::interval(write_interval);
    loop {
        log::debug!("Writer {}: waiting for next schedule", node_idx);
        interval.tick().await;
        log::debug!("Writer {}: starting write", node_idx);

        if let Ok(WorkerRequest::Stop()) = rx_writers_control.try_recv() {
            log::debug!("Writer {}: ending", node_idx);
            tx_incoming_writes
                .send(WriterReply::Completed { node_idx })
                .await
                .expect("Writer completion message couldn't be sent");
            break;
        }

        let uuid = Uuid::new_v4();
        let mut writer = writer.clone();
        let tx_incoming_writers = tx_incoming_writes.clone();
        spawn(async move {
            log::debug!("Writer {}: starting write {}", node_idx, uuid);
            let write_start = Instant::now();
            let result = writer.write(uuid).await;
            let write_duration = write_start.elapsed();
            match result {
                Ok(()) => {
                    log::debug!("Writer {}: write {} successful", node_idx, uuid);
                    tx_incoming_writers
                        .send(WriterReply::SuccessfulWrite {
                            write_start,
                            write_duration,
                            uuid,
                            node_idx,
                        })
                        .await
                        .expect("Receiver closed");
                }
                Err(ref bft_error) => {
                    log::error!("Writer {}: write {} failed: {}", node_idx, uuid, bft_error);
                    tx_incoming_writers
                        .send(WriterReply::FailedWrite {
                            write_duration,
                            uuid,
                            node_idx,
                        })
                        .await
                        .expect("Receiver closed");
                }
            };
            result
        });
    }
}
