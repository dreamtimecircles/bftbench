use std::error::Error;
use std::fmt::Display;

use async_trait::async_trait;

use uuid::Uuid;

#[derive(Debug)]
pub struct BftError {
    pub message: String
}

impl Display for BftError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl Error for BftError {}

pub type Result<T> = core::result::Result<T, BftError>;

#[async_trait]
pub trait BftBinding {
    async fn write(key: &Uuid, value: &str);
    async fn read() -> Result<Uuid>;
}
