use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;

use bft_bench_core::{
    BftBinding, BftError, BftReader, BftWriter, Node, NodeAccess, NodeEndpoint, ReadWriteNode,
    Result, WriteNode,
};
use tokio::sync::broadcast::{channel, Receiver, Sender};

pub struct ShortCircuitedBftBinding {
    sender: Arc<tokio::sync::Mutex<Sender<[u8; 16]>>>,
    receivers: HashMap<NodeEndpoint, Arc<tokio::sync::Mutex<Receiver<[u8; 16]>>>>,
}

#[derive(Clone)]
pub struct Writer {
    sender: Arc<tokio::sync::Mutex<Sender<[u8; 16]>>>,
}

#[derive(Clone)]
pub struct Reader {
    receiver: Arc<tokio::sync::Mutex<Receiver<[u8; 16]>>>,
}

#[async_trait]
impl BftBinding for ShortCircuitedBftBinding {
    type Writer = Writer;

    type Reader = Reader;

    fn new() -> Self {
        let (send, _) = channel::<[u8; 16]>(1024);
        ShortCircuitedBftBinding {
            sender: Arc::new(tokio::sync::Mutex::new(send)),
            receivers: HashMap::new(),
        }
    }

    async fn access(&mut self, node: Node) -> NodeAccess<Self::Writer, Self::Reader> {
        match node {
            Node::Write(WriteNode { endpoint: _ }) => NodeAccess::WriteOnlyAccess {
                writer: Writer {
                    sender: self.sender.clone(),
                },
            },
            Node::ReadWrite(ReadWriteNode {
                node: WriteNode { endpoint },
            }) => NodeAccess::ReadWriteAccess {
                writer: Writer {
                    sender: self.sender.clone(),
                },
                reader: Reader {
                    receiver: self
                        .receivers
                        .entry(endpoint)
                        .or_insert(Arc::new(tokio::sync::Mutex::new(
                            self.sender.lock().await.subscribe(),
                        )))
                        .clone(),
                },
            },
        }
    }
}

#[async_trait]
impl BftWriter for Writer {
    async fn write(&mut self, key: [u8; 16], _value: Arc<Vec<u8>>) -> Result<()> {
        match self.sender.lock().await.send(key) {
            Ok(_) => Ok(()),
            Err(error) => Err(BftError::dynamic(format!(
                "Error sending to the channel: {}",
                error
            ))),
        }
    }
}

#[async_trait]
impl BftReader for Reader {
    async fn read(&mut self) -> Result<[u8; 16]> {
        match self.receiver.lock().await.recv().await {
            Ok(uuid) => Ok(uuid),
            _ => Err(BftError::fixed("All senders dropped")),
        }
    }
}
