use async_trait::async_trait;

use uuid::Uuid;
use bytes::Bytes;

use {crate::config::*, crate::result::*};

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

#[async_trait]
pub trait BftWriter: Send + Clone {
    async fn write(&mut self, key: Uuid, value: Bytes) -> Result<()>;
}

#[async_trait]
pub trait BftReader: Send + Clone {
    async fn read(&mut self) -> Result<Uuid>;
}
