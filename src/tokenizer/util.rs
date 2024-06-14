use std::collections::HashMap;

use super::TokenType;

pub fn char_to_digit(ch: char) -> i32 {
    if !ch.is_ascii_digit() {
        panic!("Can't convert '{}' to digit", ch)
    };

    ch as i32 - 0x30
}

pub fn keyword_to_token_type(name: &str) -> Option<TokenType> {
    let mut lookup = HashMap::new();

    lookup.insert("module", TokenType::Module);
    lookup.insert("param", TokenType::Param);
    lookup.insert("result", TokenType::Result);
    lookup.insert("export", TokenType::Export);
    lookup.insert("const", TokenType::Const);

    lookup.insert("get", TokenType::Get);
    lookup.insert("set", TokenType::Set);

    // Types
    lookup.insert("i32", TokenType::I32);
    lookup.insert("i64", TokenType::I64);
    lookup.insert("f32", TokenType::F32);
    lookup.insert("f64", TokenType::F64);
    lookup.insert("v128", TokenType::V128);
    lookup.insert("funcref", TokenType::FuncRef);
    lookup.insert("externref", TokenType::ExternRef);
    lookup.insert("func", TokenType::Func);
    lookup.insert("extern", TokenType::Extern);
    lookup.insert("mut", TokenType::Mut);

    lookup.insert("inf", TokenType::Inf);
    lookup.insert("nan", TokenType::Nan);

    lookup.insert("local", TokenType::Local);

    lookup.get(name).cloned()
}

#[cfg(test)]
mod tests {
    use crate::tokenizer::util::char_to_digit;

    #[test]
    fn test_0_to_digit() {
        assert_eq!(char_to_digit('0'), 0)
    }

    #[test]
    fn test_1_to_digit() {
        assert_eq!(char_to_digit('1'), 1)
    }

    #[test]
    fn test_2_to_digit() {
        assert_eq!(char_to_digit('2'), 2)
    }

    #[test]
    fn test_9_to_digit() {
        assert_eq!(char_to_digit('9'), 9)
    }
}
