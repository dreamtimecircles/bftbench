use std::sync::Arc;

use async_trait::async_trait;

use bft_bench_core::{BftError, BftReader, BftWriter, NodeEndpoint, Result};
use tokio::sync::mpsc::{Receiver, Sender};

struct ShortCircuitedBftWriter {
    sender: Sender<[u8; 16]>,
}

struct ShortCircuitedBftReader {
    receiver: Receiver<[u8; 16]>,
}

#[async_trait]
impl BftWriter for ShortCircuitedBftWriter {
    async fn write(
        &mut self,
        _write_endpoint: Arc<NodeEndpoint>,
        key: [u8; 16],
        _value: Arc<Vec<u8>>,
    ) -> Result<()> {
        match self.sender.send(key).await {
            Ok(_) => Ok(()),
            Err(error) => Err(BftError::dynamic(format!(
                "Error sending to the channel: {}",
                error
            ))),
        }
    }
}

#[async_trait]
impl BftReader for ShortCircuitedBftReader {
    async fn read(&mut self, _read_endpoint: &NodeEndpoint) -> Result<[u8; 16]> {
        match self.receiver.recv().await {
            Some(uuid) => Ok(uuid),
            _ => Err(BftError::fixed("Nothing received")),
        }
    }
}
