use error::TokenizerError;
use token_store::TokenStore;

use crate::{
    traits::page_position::{PageCursor, PagePosition},
    Source,
};

mod error;
mod token_store;

pub struct Token {
    pub token_type: TokenType,
    pub cursor: PageCursor,
}

impl PagePosition for Token {
    fn position(&self) -> PageCursor {
        self.cursor
    }
}

pub enum TokenType {
    LeftParen,
    RightParen,
    LineComment(String),
}

pub fn generate_tokens(input: Source) -> Result<TokenStore, Vec<TokenizerError>> {
    let mut store = TokenStore::default();
    let mut errors: Vec<TokenizerError> = Vec::new();

    let input_iter = &mut input.into_iter().peekable();

    loop {
        let (character, cursor) = match input_iter.next() {
            Some(res) => res,
            None => break,
        };

        match character {
            '(' => store.tokens.push(Token {
                token_type: TokenType::LeftParen,
                cursor,
            }),
            ')' => store.tokens.push(Token {
                token_type: TokenType::RightParen,
                cursor,
            }),
            ';' => {
                if let Some(_) = &input_iter.next_if(|(char, _)| *char == ';') {
                    let comment_contents = input_iter
                        .take_while(|(char, _)| *char != '\n')
                        .map(|(char, _)| char)
                        .collect::<String>()
                        .clone();

                    store.tokens.push(Token {
                        token_type: TokenType::LineComment(comment_contents),
                        cursor,
                    })
                }
            }
            char if char.is_ascii_alphabetic() => {}
            ' ' | '\n' => {}
            _ => errors.push(TokenizerError {
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
