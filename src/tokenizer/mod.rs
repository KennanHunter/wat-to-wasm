use error::TokenizerError;
use token_store::TokenStore;
use util::char_to_digit;

use crate::{
    source::SourceIter,
    traits::page_position::{PageCursor, PagePosition},
    Source,
};

mod error;
mod tests;
mod util;
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
    String(String),
    Integer(i32),
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

        if character.is_ascii_whitespace() {
            continue;
        }

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

        number_start if number_start.is_ascii_digit() => {
            let number = source_iter
                .take_while(|(char, _)| char.is_ascii_digit() || *char == '_')
                .fold(char_to_digit(number_start), |prev, (digit, _)| {
                    if digit == '_' {
                        return prev;
                    };

                    prev * 10 + char_to_digit(digit)
                });

            Some(TokenType::Integer(number))
        }
        sign_char if sign_char == '+' || sign_char == '-' => {
            let number = source_iter
                .take_while(|(char, _)| char.is_ascii_digit() || *char == '_')
                .fold(0, |prev, (digit, _)| {
                    if digit == '_' {
                        return prev;
                    };

                    prev * 10 + char_to_digit(digit)
                });

            if sign_char == '+' {
                Some(TokenType::Integer(number))
            } else {
                Some(TokenType::Integer(-number))
            }
        }

        '"' => {
            // TODO: Support hex digits and escape characters in strings
            let string_contents = source_iter
                .take_while(|(char, _)| *char != '"')
                .map(|(char, _)| char)
                .collect::<String>();

            Some(TokenType::String(string_contents))
        }

        char if char.is_ascii_alphabetic() => {
            // Keyword

            None
        }
        _ => None,
    }
}
