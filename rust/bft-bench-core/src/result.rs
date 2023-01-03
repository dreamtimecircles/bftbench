use std::{borrow::Cow, error::Error, fmt::Display};

#[derive(Debug)]
pub struct BftError {
    message: Cow<'static, str>,
}

impl Error for BftError {}

impl BftError {
    pub fn fixed(message: &'static str) -> Self {
        BftError {
            message: Cow::Borrowed(message),
        }
    }

    pub fn dynamic(message: String) -> Self {
        BftError {
            message: Cow::Owned(message),
        }
    }
}

impl Display for BftError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

pub type Result<T> = core::result::Result<T, BftError>;
