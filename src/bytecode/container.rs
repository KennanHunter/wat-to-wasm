use std::fmt::Debug;

use super::{MAGIC_BYTES, WASM_VERSION};

pub struct BytecodeContainer {
    pub data: Vec<u8>,
}

impl Default for BytecodeContainer {
    fn default() -> Self {
        Self {
            data: Default::default(),
        }
    }
}

impl Debug for BytecodeContainer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("BytecodeContainer = {\n")?;

        f.write_str("\tBytecode ")?;

        f.write_str(&format!("{:#X?}", self.data))?;

        f.write_str("\n}")
    }
}

impl From<Vec<u8>> for BytecodeContainer {
    fn from(value: Vec<u8>) -> Self {
        BytecodeContainer { data: value }
    }
}

impl BytecodeContainer {
    pub fn push_byte(&mut self, data: u8) {
        self.data.push(data);
    }

    pub fn push_bytes(&mut self, bytes: &[u8]) {
        self.data.extend(bytes);
    }

    pub fn new_module() -> Self {
        let mut bytecode = Self::default();

        bytecode.push_bytes(&MAGIC_BYTES);
        bytecode.push_bytes(&WASM_VERSION);

        bytecode
    }
}
