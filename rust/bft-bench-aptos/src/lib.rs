use async_trait::async_trait;
use uuid::Uuid;

use bft_bench_core::{BftBinding, Result, BftError};

struct AptosBftBinding {}

#[async_trait]
impl BftBinding for AptosBftBinding {
    async fn write(_key: &Uuid, _value: &str) {

    }

    async fn read() -> Result<Uuid> {
        Err(BftError { message: "todo".to_string() })
    }
}
