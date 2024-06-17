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

impl IntoIterator for TokenStore {
    type Item = Token;
    type IntoIter = TokenIter;

    fn into_iter(self) -> Self::IntoIter {
        TokenIter {
            token_iter: self.tokens,
            cursor: 0,
        }
    }
}

pub struct TokenIter {
    token_iter: Vec<Token>,
    cursor: usize,
}

impl Iterator for TokenIter {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
