use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;

use bft_bench_core::{
    BftBinding, BftError, BftReader, BftWriter, Node, NodeAccess, NodeEndpoint, ReadWriteNode,
    Result, WriteNode,
};
use tokio::sync::broadcast::{channel, Receiver, Sender};

const CHANNEL_BUFFER_SIZE: usize = 1024 * 1024;

pub struct ShortCircuitedBftBinding {
    writer: Writer,
    first_reader: Option<Reader>,
    readers: HashMap<NodeEndpoint, Reader>,
}

#[derive(Clone)]
pub struct Writer {
    sender: Sender<[u8; 16]>,
}

pub struct Reader {
    receiver_factory: Sender<[u8; 16]>,
    receiver: Receiver<[u8; 16]>,
}

impl Clone for Reader {
    fn clone(&self) -> Self {
        Self {
            receiver_factory: self.receiver_factory.clone(),
            receiver: self.receiver_factory.subscribe(),
        }
    }
}

#[async_trait]
impl BftBinding for ShortCircuitedBftBinding {
    type Writer = Writer;

    type Reader = Reader;

    fn new() -> Self {
        let (sender, receiver) = channel::<[u8; 16]>(CHANNEL_BUFFER_SIZE);
        let sender_for_reader = sender.clone();
        ShortCircuitedBftBinding {
            writer: Writer { sender },
            // We store and reuse the first reader for the first node (see `access` below) so that there's no
            // chance of receiving errors when sending/receiving due to all opposite handles being closed.
            first_reader: Some(Reader {
                receiver_factory: sender_for_reader,
                receiver,
            }),
            readers: HashMap::new(),
        }
    }

    async fn access(&mut self, node: Node) -> NodeAccess<Self::Writer, Self::Reader> {
        match node {
            Node::Write(WriteNode { endpoint: _ }) => NodeAccess::WriteOnlyAccess {
                writer: self.writer.clone(),
            },
            Node::ReadWrite(ReadWriteNode {
                node: WriteNode { endpoint },
            }) => NodeAccess::ReadWriteAccess {
                writer: self.writer.clone(),
                reader: self
                    .readers
                    .entry(endpoint)
                    .or_insert(match self.first_reader.take() {
                        Some(reader) => reader,
                        None => Reader {
                            receiver_factory: self.writer.sender.clone(),
                            receiver: self.writer.sender.subscribe(),
                        },
                    })
                    .clone(),
            },
        }
    }
}

#[async_trait]
impl BftWriter for Writer {
    async fn write(&mut self, key: [u8; 16], _value: Arc<Vec<u8>>) -> Result<()> {
        match self.sender.send(key) {
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
        match self.receiver.recv().await {
            Ok(uuid) => Ok(uuid),
            Err(error) => Err(BftError::dynamic(format!(
                "Error receiving from the channel: {}",
                error
            ))),
        }
    }
}
