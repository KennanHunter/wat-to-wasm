use error::TokenizerError;
use token_store::TokenStore;

use crate::{
    source::SourceIter,
    traits::page_position::{PageCursor, PagePosition},
    Source,
};

mod error;
mod tests;
#[macro_use]
mod token_store;

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub cursor: PageCursor,
}

impl PagePosition for Token {
    fn position(&self) -> PageCursor {
        self.cursor
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType {
    LeftParen,
    RightParen,
    SemiColon,
    LineComment(String),
}

pub fn generate_tokens(input: Source) -> Result<TokenStore, Vec<TokenizerError>> {
    let mut store = TokenStore::default();
    let mut errors: Vec<TokenizerError> = Vec::new();

    let input_iter = &mut input.into_iter();

    loop {
        let (character, cursor) = match input_iter.next() {
            Some(res) => res,
            None => break,
        };

        match tokenize_token(input_iter, character) {
            Some(token_type) => store.tokens.push(Token { token_type, cursor }),
            None => errors.push(TokenizerError {
                unrecognized_character: character,
                cursor,
            }),
        }
    }

    if errors.len() > 0 {
        Err(errors)
    } else {
        Ok(store)
    }
}

fn tokenize_token(source_iter: &mut SourceIter, character: char) -> Option<TokenType> {
    match character {
        '(' => Some(TokenType::LeftParen),
        ')' => Some(TokenType::RightParen),

        ';' => {
            if source_iter.next_if_char(';').is_some() {
                let comment_contents = source_iter
                    .take_while(|(char, _)| *char != '\n')
                    .map(|(char, _)| char)
                    .collect::<String>();

                Some(TokenType::LineComment(comment_contents))
            } else {
                Some(TokenType::SemiColon)
            }
        }

        char if char.is_ascii_whitespace() => None,
        char if char.is_ascii_whitespace() => None,

        _ => None,
    }
}
