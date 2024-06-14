use error::TokenizerError;
use token_store::TokenStore;
use util::{char_to_digit, keyword_to_token_type};

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
    IntegerLiteral(i32),
    Identifier(String),
    I32,
    I64,
    F32,
    F64,
    V128,
    FuncRef,
    ExternRef,
    Func,
    Extern,
    Module,
    Result,
    Param,
    Mut,
    Local,
    Dot,
    Set,
    Get,
    Export,
    Const,
    Nan,
    Inf,
}

fn is_identifier_character(ch: char) -> bool {
    match ch {
        alphanumeric if alphanumeric.is_ascii_alphanumeric() => true,
        '!' | '#' | '$' | '%' | '&' | '′' | '*' | '+' | '-' | '.' | '/' | ':' | '<' | '=' | '>'
        | '?' | '@' | '\\' | '^' | '_' | '`' | '|' | '~' => true,
        _ => false,
    }
}

fn is_separator_character(ch: char) -> bool {
    match ch {
        whitespace if whitespace.is_ascii_whitespace() => true,
        '.' | ')' | ';' => true,
        _ => false,
    }
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

        '$' => {
            let identifier_name: String =
                source_iter.consume_to_string_while(|(ch, _)| is_identifier_character(ch));

            if identifier_name.len() == 0 {
                // TODO: ensure that empty identifiers can't happen
            }

            Some(TokenType::Identifier(identifier_name))
        }

        number_start if number_start.is_ascii_digit() => {
            let number = source_iter
                .consume_to_string_while(|(char, _)| char.is_ascii_digit() || char == '_')
                .chars()
                .fold(char_to_digit(number_start), |prev, digit| {
                    if digit == '_' {
                        return prev;
                    };

                    prev * 10 + char_to_digit(digit)
                });

            Some(TokenType::IntegerLiteral(number))
        }
        sign_char if sign_char == '+' || sign_char == '-' => {
            let number = source_iter
                .consume_to_string_while(|(char, _)| char.is_ascii_digit() || char == '_')
                .chars()
                .fold(0, |prev, digit| {
                    if digit == '_' {
                        return prev;
                    };

                    prev * 10 + char_to_digit(digit)
                });

            if sign_char == '+' {
                Some(TokenType::IntegerLiteral(number))
            } else {
                Some(TokenType::IntegerLiteral(-number))
            }
        }

        '"' => {
            // TODO: Support hex digits and escape characters in strings
            let string_contents = source_iter.consume_to_string_while(|(char, _)| char != '"');

            source_iter.expect('"').expect("\" should follow string");

            Some(TokenType::String(string_contents))
        }

        '.' => Some(TokenType::Dot),

        keyword_start if keyword_start.is_ascii_alphabetic() => {
            let keyword = keyword_start.to_string()
                + &source_iter.consume_to_string_while(|(char, _)| !is_separator_character(char));

            keyword_to_token_type(&keyword)
        }

        _ => None,
    }
}
