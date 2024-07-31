use crate::{
    parser::errors::{
        ExpectedIdentifierError, ExpectedStringError, ExpectedTokenError, ExpectedTypeError,
    },
    shared::Identifier,
    traits::{error_display::ErrorDisplay, page_position::PageCursor},
};

use super::{Token, TokenType};

#[derive(Debug, PartialEq, Default)]
pub struct TokenStore {
    pub tokens: Vec<Token>,
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

impl TokenIter {
    pub fn peek(&self) -> Option<Token> {
        self.token_iter.get(self.cursor).cloned()
    }

    pub fn guess_cursor(&self) -> Option<PageCursor> {
        // TODO: Make this smarter... detect when cursor should advance
        self.token_iter
            .get(self.cursor - 1)
            .map(|token| token.cursor)
    }

    pub fn consume_identifier(
        &mut self,
    ) -> Result<(Identifier, PageCursor), ExpectedIdentifierError> {
        match self.peek() {
            Some(token) => match token.token_type {
                TokenType::Identifier(id) => {
                    self.next();

                    Ok((id, token.cursor))
                }
                _ => Err(ExpectedIdentifierError {
                    cursor: token.cursor,
                }),
            },
            None => todo!(),
        }
    }

    pub fn consume_type(&mut self) -> Result<Token, ExpectedTypeError> {
        match self.peek() {
            Some(token) => match &token.token_type {
                TokenType::I32 | TokenType::I64 | TokenType::F32 | TokenType::F64 => {
                    self.next();

                    Ok(token)
                }
                _ => Err(ExpectedTypeError {
                    cursor: token.cursor,
                }),
            },
            None => todo!(),
        }
    }

    pub fn consume_string(&mut self) -> Result<(String, PageCursor), ExpectedStringError> {
        match self.peek() {
            Some(token) => match token.token_type {
                TokenType::String(literal) => {
                    self.next();

                    Ok((literal, token.cursor))
                }
                _ => Err(ExpectedStringError {
                    cursor: token.cursor,
                }),
            },
            None => todo!(),
        }
    }

    /// Only advances if the next token matches the `expected_token_type`
    pub fn consume(
        &mut self,
        expected_token_type: TokenType,
    ) -> Result<Token, Box<dyn ErrorDisplay>> {
        if self
            .peek()
            .is_some_and(|token| token.token_type == expected_token_type)
        {
            Ok(self.next().unwrap())
        } else {
            Err(Box::new(ExpectedTokenError {
                expected_token: expected_token_type,
                cursor: self
                    .guess_cursor()
                    .expect("should be able to estimate Cursor to display ExpectedTokenError"),
            }))
        }
    }
}

impl Iterator for TokenIter {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let res = self.token_iter.get(self.cursor).cloned();

        self.cursor += 1;

        res
    }
}
