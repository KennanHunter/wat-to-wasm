enum InstructionType {
    Add,
    Sub,
    Mul,
    Div(SignedAware),
}

enum SignedAware {
    Signed,
    Unsigned,
}
