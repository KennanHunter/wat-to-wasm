use crate::shared::Identifier;

pub struct UnaryOperation {
    operand: Expr,
}

#[derive(Default, Debug)]
pub struct Module {
    pub exprs: Vec<Expr>,
}

#[derive(Debug)]
pub struct FunctionDefinition {
    id: Option<Identifier>,
}

#[derive(Debug)]
pub struct Export {
    pub name: String,
    pub expr: Expr,
}

#[derive(Debug)]
pub enum Expr {
    Module(Module),
    Func(FunctionDefinition),
    FuncReference(Identifier),
    Export(Box<Export>),
}
