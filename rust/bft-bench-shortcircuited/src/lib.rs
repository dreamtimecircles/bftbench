use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;

use bft_bench_core::{
    BftBinding, BftError, BftReader, BftWriter, Node, NodeAccess, NodeEndpoint, ReadWriteNode,
    Result, WriteNode,
};
use tokio::sync::broadcast::{channel, Receiver, Sender};

pub struct ShortCircuitedBftBinding {
    writer: Writer,
    readers: HashMap<NodeEndpoint, Reader>,
}

// Note that we wrap both `Sender` and `Receiver` in an `Arc` so that we can `clone` it, getting a new
// reference-counted owning pointer that can be moved to a free-running async; since free-running asyncs
// have `'static` lifetime (i.e., not known at compile time), `Arc`s moved to them live as long
// as they live, thus a channel end will live as long as the longest-living free-running async
// holding an `Arc` to it.

#[derive(Clone)]
pub struct Writer {
    sender: Arc<Sender<[u8; 16]>>,
}

#[derive(Clone)]
pub struct Reader {
    // Since `recv` is `&mut self`, we must use a mutex to share it via `Arc`s; furthermore,
    // `recv` is also `async`, so we can't use a regular mutex but we need a tokio one,
    // so that it is `Send` and can thus be moved between threads and thus work across
    // `await`s.
    // That's OK though, as it's used for a single node and we'll be anyway reading from each
    // node in a loop, not concurrently.
    receiver: Arc<tokio::sync::Mutex<Receiver<[u8; 16]>>>,
}

#[async_trait]
impl BftBinding for ShortCircuitedBftBinding {
    type Writer = Writer;

    type Reader = Reader;

    fn new() -> Self {
        // Throwing away the first read end because we don't know who the readers are, yet.
        let (send, _) = channel::<[u8; 16]>(1024);
        ShortCircuitedBftBinding {
            writer: Writer {
                sender: Arc::new(send),
            },
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
                // Clone the `Arc` and move it to the client of the BFT binding for it to use clone as necessary.
                writer: self.writer.clone(),
                // Insert a new `Arc`, so that the read end lives at least as long as the map, and thus the BFT binding, does,
                // then clone the `Arc` and move it to the client of the BFT binding for it to use and clone as necessary.
                reader: self
                    .readers
                    .entry(endpoint)
                    .or_insert(Reader {
                        receiver: Arc::new(tokio::sync::Mutex::new(self.writer.sender.subscribe())),
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
        match self.receiver.lock().await.recv().await {
            Ok(uuid) => Ok(uuid),
            _ => Err(BftError::fixed("No senders alive")),
        }
    }
}
