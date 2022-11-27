use std::sync::Arc;

use async_trait::async_trait;

use bft_bench_core::{BftBinding, NodeEndpoint, Result};

struct AptosBftBinding {}

#[async_trait]
impl BftBinding for AptosBftBinding {
    async fn write(write_endpoint: Arc<NodeEndpoint>, key: [u8; 16], value: Arc<Vec<u8>>) -> Result<()> {
        todo!()
    }
    async fn read(read_endpoint: &NodeEndpoint) -> Result<[u8; 16]> {
        todo!()
    }
}
