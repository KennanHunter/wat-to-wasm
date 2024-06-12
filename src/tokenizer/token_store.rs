use super::Token;

pub struct TokenStore {
    pub tokens: Vec<Token>,
}

impl Default for TokenStore {
    fn default() -> Self {
        Self { tokens: Vec::new() }
    }
}
