pub mod errors;

use std::collections::HashMap;

use errors::MisplacedError;

use crate::{
    bytecode::container::BytecodeContainer,
    parser::expression::{Expr, FunctionDefinition, Module},
    shared::Identifier,
    traits::{error_display::ErrorDisplay, page_position::PageCursor},
};

struct BytecodeModule {
    pub functions: HashMap<Identifier, FunctionDefinition>,
    pub exports: Vec<Identifier>,
}

impl BytecodeModule {
    fn new() -> Self {
        Self {
            functions: Default::default(),
            exports: Default::default(),
        }
    }
}

pub fn compile(tree: Expr) -> Result<BytecodeContainer, Box<dyn ErrorDisplay>> {
    let Expr::Module(Module { exprs }) = tree else {
        todo!("Currently does not support compiling non modules");
    };

    let mut bc = BytecodeModule::new();

    for expr in exprs {
        match expr {
            Expr::Func(definition) => {
                bc.functions.insert(
                    definition
                        .id
                        .clone()
                        // TODO: Gracefully handle this error
                        .expect("Top level function definitions should have identifiers"),
                    definition,
                );
            }

            Expr::Export(_) => todo!(),

            _ => {
                return Err(Box::new(MisplacedError {
                    cursor: PageCursor::start(),
                }))
            }
        }
    }

    Ok(Default::default())
}
