pub mod errors;
pub mod expression;
mod instructions;
mod rules;
use std::vec;

use errors::ExpectedMethodError;
use expression::{Export, Expr, FunctionDefinition, IntOp, Module, Param};
use instructions::BuiltinType;

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
        let token = tokens.next().unwrap();

        return match token.token_type {
            TokenType::LineComment(_) => continue,

            // TODO: Better figure out scoping
            TokenType::LeftParen => continue,
            TokenType::Module => Ok(Expr::Module(Module {
                exprs: parse_multiple_expressions(tokens)?,
            })),

            TokenType::Func => {
                let (id, _) = tokens.consume_identifier()?;

                if tokens.consume(TokenType::RightParen).is_ok() {
                    return Ok(Expr::FuncReference(id));
                }

                let body = parse_multiple_expressions(tokens)?;

                Ok(Expr::Func(FunctionDefinition { id: Some(id), body }))
            }

            TokenType::Export => {
                let (name, _) = tokens.consume_string()?;

                Ok(Expr::Export(Box::new(Export {
                    name,
                    expr: parse_expression(tokens)?,
                })))
            }

            TokenType::Param => {
                let (id, _) = tokens.consume_identifier()?;

                let parameter_type = match tokens.consume_type()?.token_type {
                        TokenType::I32 => BuiltinType::I32,
                        TokenType::F32 => BuiltinType::F32,
                        TokenType::I64 => BuiltinType::I64,
                        TokenType::F64 => BuiltinType::F64,

                        _ => unreachable!("Every token that consume_type returns should be convertible to a BuiltinType"),
                };

                tokens.consume(TokenType::RightParen)?;

                Ok(Expr::Param(Box::new(Param { id, parameter_type })))
            }

            TokenType::Result => {
                let parameter_type = match tokens.consume_type()?.token_type {
                    TokenType::I32 => BuiltinType::I32,
                    TokenType::F32 => BuiltinType::F32,
                    TokenType::I64 => BuiltinType::I64,
                    TokenType::F64 => BuiltinType::F64,

                    _ => unreachable!("Every token that consume_type returns should be convertible to a BuiltinType"),
                };

                tokens.consume(TokenType::RightParen)?;

                Ok(Expr::Result(parameter_type))
            }

            TokenType::Local => {
                tokens.consume(TokenType::Dot)?;

                let method = tokens.next().unwrap();

                let (id, _) = tokens.consume_identifier()?;

                tokens.consume(TokenType::RightParen)?;

                match method.token_type {
                    TokenType::Get => Ok(Expr::LocalGet(id)),

                    _ => Err(Box::new(ExpectedMethodError {
                        cursor: method.cursor,
                        methods: vec!["get"],
                    })),
                }
            }

            TokenType::I32 => {
                tokens.consume(TokenType::Dot)?;

                let method = tokens.next().unwrap();

                match method.token_type {
                    TokenType::Add => Ok(Expr::IntOp(BuiltinType::I32, IntOp::Add)),
                    _ => Err(Box::new(ExpectedMethodError {
                        cursor: method.cursor,
                        methods: vec!["add"],
                    })),
                }
            }

            _ => todo!("parsing of {:?}", token.token_type),
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
