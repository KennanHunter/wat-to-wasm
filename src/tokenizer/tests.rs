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
        res.unwrap(),
        (TokenStore {
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

#[test]
fn test_tokenize_string() {
    let source: Source = "\"string contents\"()".into();

    let res = generate_tokens(dbg!(source));

    assert_eq!(
        res.unwrap(),
        (TokenStore {
            tokens: vec![
                Token {
                    token_type: TokenType::String("string contents".to_string()),
                    cursor: PageCursor { line: 1, column: 0 }
                },
                Token {
                    token_type: TokenType::LeftParen,
                    cursor: PageCursor {
                        line: 1,
                        column: 17
                    }
                },
                Token {
                    token_type: TokenType::RightParen,
                    cursor: PageCursor {
                        line: 1,
                        column: 18
                    }
                }
            ]
        })
    );
}

#[test]
fn test_parse_number() {
    let source: Source = "1234".into();

    let res = generate_tokens(source);

    assert_eq!(
        res.unwrap(),
        (TokenStore {
            tokens: vec![Token {
                token_type: TokenType::IntegerLiteral(1234),
                cursor: PageCursor::start()
            }]
        })
    )
}

#[test]
fn test_parse_number_with_underscore() {
    let source: Source = "1_234".into();

    let res = generate_tokens(source);

    assert_eq!(
        res.unwrap(),
        (TokenStore {
            tokens: vec![Token {
                token_type: TokenType::IntegerLiteral(1234),
                cursor: PageCursor::start()
            }]
        })
    )
}

#[test]
fn test_parse_number_with_negative() {
    let source: Source = "-1_234".into();

    let res = generate_tokens(source);

    assert_eq!(
        res.unwrap(),
        (TokenStore {
            tokens: vec![Token {
                token_type: TokenType::IntegerLiteral(-1234),
                cursor: PageCursor::start()
            }]
        })
    )
}

#[test]
fn test_parse_identifier_name() {
    let source: Source = "$epic-identifier>=<&@!%^&)".into();

    let res = generate_tokens(source);

    assert_eq!(
        res.unwrap(),
        (TokenStore {
            tokens: vec![
                Token {
                    token_type: TokenType::Identifier(
                        "epic-identifier>=<&@!%^&".to_string().into()
                    ),
                    cursor: PageCursor::start()
                },
                Token {
                    token_type: TokenType::RightParen,
                    cursor: PageCursor {
                        column: 25,
                        line: 1,
                    }
                }
            ]
        })
    )
}
