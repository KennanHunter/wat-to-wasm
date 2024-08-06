pub enum InstructionType {
    Add,
    Sub,
    Mul,
    Div(SignedAware),
}

#[derive(Debug, Clone)]
pub enum BuiltinType {
    I32,
    F32,
    I64,
    F64,
}

enum SignedAware {
    Signed,
    Unsigned,
}
