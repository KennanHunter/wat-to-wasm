use super::Token;

#[derive(Debug, PartialEq)]
pub struct TokenStore {
    pub tokens: Vec<Token>,
}

impl Default for TokenStore {
    fn default() -> Self {
        Self { tokens: Vec::new() }
    }
}

#[macro_export]
macro_rules! tokens {
    () => {
        TokenStore::default()
    }; // TODO: Emulate Vec! macro behavior
}
