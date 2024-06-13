#![cfg(test)]
use crate::{source::Source, tokenizer::generate_tokens};
use crate::{
    tokenizer::{token_store::TokenStore, Token, TokenType},
    traits::page_position::PageCursor,
};

#[test]
fn test_tokenize_line_comment() {
    let source: Source = ";; comment contents\n()".into();

    let res = generate_tokens(dbg!(source));

    assert_eq!(
        res,
        Ok(TokenStore {
            tokens: vec![
                Token {
                    token_type: TokenType::LineComment(" comment contents".to_string()),
                    cursor: PageCursor { line: 1, column: 0 }
                },
                Token {
                    token_type: TokenType::LeftParen,
                    cursor: PageCursor { line: 2, column: 0 }
                },
                Token {
                    token_type: TokenType::RightParen,
                    cursor: PageCursor { line: 2, column: 1 }
                }
            ]
        })
    );
}
