pub mod errors;
mod expression;
mod instructions;
mod rules;
use expression::{Export, Expr, Module};

use crate::{
    tokenizer::{
        token_store::{TokenIter, TokenStore},
        TokenType,
    },
    traits::error_display::ErrorDisplay,
};

type Tokens<'a> = &'a mut TokenIter;

pub fn parse_tokens(tokens: TokenStore) -> Result<Expr, Box<dyn ErrorDisplay>> {
    parse_expression(&mut tokens.into_iter())
}

fn parse_expression(tokens: Tokens) -> Result<Expr, Box<dyn ErrorDisplay>> {
    loop {
        let token = dbg!(tokens.next().unwrap());

        return match token.token_type {
            // TODO: Better figure out scoping
            TokenType::LineComment(_) => continue,

            TokenType::LeftParen => parse_expression(tokens),
            TokenType::Module => Ok(Expr::Module(Module {
                exprs: parse_multiple_expressions(tokens)?,
            })),

            TokenType::Func => {
                let id = match tokens.consume_identifier() {
                    Ok((id, _)) => id,
                    Err(err) => return Err(Box::new(err)),
                };

                if tokens.consume(TokenType::RightParen).is_ok() {
                    return Ok(Expr::FuncReference(id));
                }

                todo!("function definitions")
            }

            TokenType::Export => {
                let name = match tokens.consume_string() {
                    Ok((literal, _)) => literal,
                    Err(err) => return Err(Box::new(err)),
                };

                Ok(Expr::Export(Box::new(Export {
                    name,
                    expr: parse_expression(tokens)?,
                })))
            }

            _ => todo!("token type"),
        };
    }
}

fn parse_multiple_expressions(tokens: Tokens) -> Result<Vec<Expr>, Box<dyn ErrorDisplay>> {
    let mut exprs = Vec::new();

    while tokens
        .peek()
        .is_some_and(|token| token.token_type != TokenType::RightParen)
    {
        exprs.push(parse_expression(tokens)?);
    }

    Ok(exprs)
}
