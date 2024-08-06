use super::{MAGIC_BYTES, WASM_VERSION};

#[derive(Default)]
pub struct BytecodeContainer {
    pub data: Vec<u8>,
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
