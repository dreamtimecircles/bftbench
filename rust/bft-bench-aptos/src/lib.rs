use std::sync::Arc;

use async_trait::async_trait;

use bft_bench_core::{BftBinding, NodeEndpoint, Result};

struct AptosBftBinding {}

#[async_trait]
impl BftBinding for AptosBftBinding {
    async fn write(_write_endpoint: Arc<NodeEndpoint>, _key: [u8; 16], _value: Arc<Vec<u8>>) -> Result<()> {
        todo!()
    }
    async fn read(_read_endpoint: &NodeEndpoint) -> Result<[u8; 16]> {
        todo!()
    }
}
