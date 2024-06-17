use crate::tokenizer::token_store::{TokenIter, TokenStore};

type Tokens<'a> = &'a mut TokenIter;

pub fn parse_tokens(mut tokens: TokenStore) {
    parse_expression(&mut tokens.into_iter());
}

fn parse_expression(tokens: Tokens) {
    todo!()
}
