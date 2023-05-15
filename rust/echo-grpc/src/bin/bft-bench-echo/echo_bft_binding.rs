use async_stream::stream;
use async_trait::async_trait;
use tokio::sync::mpsc::{channel, Sender};
use tokio_stream::StreamExt;
use tonic::Streaming;
use uuid::{Bytes, Uuid};

use bft_bench_core::{
    BftBinding, BftError, BftReader, BftWriter, Config, Node, NodeAccess, ReadWriteNode, Result,
    WriteNode,
};

pub mod pb {
    tonic::include_proto!("grpc.echo");
}

const CHANNEL_BUFFER_SIZE: usize = 128 * 1024 * 1024;

pub struct EchoBftBinding {}

#[derive(Clone)]
pub struct Writer {
    sender: Sender<pb::Tx>,
}

pub struct Reader {
    response_stream: Streaming<pb::Tx>,
}

#[async_trait]
impl BftBinding for EchoBftBinding {
    type Writer = Writer;

    type Reader = Reader;

    fn new(config: &Config) -> Self {
        if config.nodes.len() != 1 {
            unsupported();
        }
        EchoBftBinding {}
    }

    async fn access(&mut self, node: Node) -> NodeAccess<Self::Writer, Self::Reader> {
        match node {
            Node::Write(WriteNode { endpoint: _ }) => {
                unsupported();
            }
            Node::ReadWrite(ReadWriteNode {
                node: WriteNode { endpoint },
            }) => {
                let (sender, mut receiver) = channel::<pb::Tx>(CHANNEL_BUFFER_SIZE);
                let in_stream = stream! {
                    while let Some(tx) = receiver.recv().await {
                        yield tx
                    }
                };

                let mut client = pb::service_client::ServiceClient::connect(format!(
                    "http://{}:{}",
                    endpoint.host, endpoint.port
                ))
                .await
                .unwrap();
                let response = client.echo(in_stream).await.unwrap();

                let response_stream = response.into_inner();

                NodeAccess::ReadWriteAccess {
                    writer: Writer { sender },
                    reader: Reader { response_stream },
                }
            }
        }
    }
}

#[async_trait]
impl BftWriter for Writer {
    async fn write(&mut self, key: Uuid) -> Result<()> {
        match self
            .sender
            .send(pb::Tx {
                tx_id: Vec::from(key.to_bytes_le()),
                value: vec![0; 1000],
            })
            .await
        {
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
    async fn read(&mut self) -> Result<Option<Uuid>> {
        match self.response_stream.next().await {
            Some(Ok(tx)) => {
                let mut arr: Bytes = [0u8; 16];
                arr[..16].copy_from_slice(&tx.tx_id[..16]);
                Ok(Some(Uuid::from_bytes_le(arr)))
            }
            Some(Err(error)) => Err(BftError::dynamic(format!(
                "Error receiving from the stream: {}",
                error
            ))),
            None => Ok(None), // stream closed
        }
    }
}

fn unsupported() -> ! {
    panic!("Only one read-write node is supported")
}
