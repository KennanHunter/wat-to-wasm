use crate::tokenizer::{
    token_store::{TokenIter, TokenStore},
    TokenType,
};

type Tokens<'a> = &'a mut TokenIter;

pub fn parse_tokens(mut tokens: TokenStore) {
    parse_expression(&mut tokens.into_iter());
}

fn parse_expression(tokens: Tokens) {
    let token = tokens.next().unwrap();

    match token.token_type {
        TokenType::LeftParen => {}
        _ => todo!(),
    }
}
