use async_trait::async_trait;

use bytes::Bytes;
use uuid::Uuid;

use {crate::config::*, crate::result::*};

/// A [`BftBinding`] can be stateful; it provides a [`new`](BftBinding::new) constructor and
/// [`access`](BftBinding::access) to a [`Node`]. Depending on the type of node, access can
/// be read-write or write-only.
#[async_trait]
pub trait BftBinding {
    type Writer: BftWriter;
    type Reader: BftReader;

    fn new() -> Self;
    async fn access(&mut self, node: Node) -> NodeAccess<Self::Writer, Self::Reader>;
}

pub enum NodeAccess<Writer: BftWriter, Reader: BftReader> {
    ReadWriteAccess { writer: Writer, reader: Reader },
    WriteOnlyAccess { writer: Writer },
}

/// A [`BftWriter`] allows to write a key-value pair to a node.
#[async_trait]
pub trait BftWriter: Send + Clone {
    async fn write(&mut self, key: Uuid, value: Bytes) -> Result<()>;
}

/// A [`BftWriter`] allows to read a key from a node. A BFT library/platform is expected to
/// provide BFT ordering functionality, i.e., all nodes should provide a consistent ordering
/// of written data to all readers, including when byzantine nodes are present.
/// 
/// BFT properties can vary but most private-network BFT protocols guarantee consistent
/// ordering when at least `2f + 1` nodes are honest, where `f` is the maximum number
/// of dishonest nodes.
#[async_trait]
pub trait BftReader: Send + Clone {
    async fn read(&mut self) -> Result<Uuid>;
}
