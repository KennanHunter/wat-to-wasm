use crate::shared::Identifier;

use super::instructions::BuiltinType;

pub struct UnaryOperation {
    operand: Expr,
}

#[derive(Default, Debug)]
pub struct Module {
    pub exprs: Vec<Expr>,
}

#[derive(Debug)]
pub struct FunctionDefinition {
    pub id: Option<Identifier>,
    pub body: Vec<Expr>,
}

#[derive(Debug)]
pub struct Export {
    pub name: String,
    pub expr: Expr,
}

#[derive(Debug)]
pub struct Param {
    pub id: Identifier,
    pub parameter_type: BuiltinType,
}

#[derive(Debug)]
pub enum Expr {
    Module(Module),
    Func(FunctionDefinition),
    FuncReference(Identifier),
    Export(Box<Export>),
    Param(Box<Param>),
    Result(BuiltinType),
    LocalGet(Identifier),
}
