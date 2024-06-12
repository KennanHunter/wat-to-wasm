use crate::{source::Source, traits::page_position::PageCursor};

#[derive(Debug, Clone)]
pub struct TokenizerError {
    pub unrecognized_character: char,
    pub cursor: PageCursor,
}

impl TokenizerError {
    pub fn display(&self, source: Source) -> String {
        let position = source
            .display_position(self.cursor)
            .expect("tokenizer error should point to valid position");

        format!(
            "Unrecognized character {} found in line:\n{}",
            self.unrecognized_character, position
        )
    }
}
