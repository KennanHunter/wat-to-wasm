use crate::{
    source::Source,
    tokenizer::TokenType,
    traits::{error_display::ErrorDisplay, page_position::PageCursor},
};

#[derive(Debug)]
pub struct ExpectedTokenError {
    pub expected_token: TokenType,
    pub cursor: PageCursor,
}

impl ErrorDisplay for ExpectedTokenError {
    fn display(&self, source: Source) -> String {
        let position = source
            .display_position(self.cursor)
            .expect("tokenizer error should point to valid position");

        format!(
            "Expected token {:?} in line:\n{}",
            self.expected_token, position
        )
    }
}

#[derive(Debug)]
pub struct ExpectedIdentifierError {
    pub cursor: PageCursor,
}

impl ErrorDisplay for ExpectedIdentifierError {
    fn display(&self, source: Source) -> String {
        let position = source
            .display_position(self.cursor)
            .expect("tokenizer error should point to valid position");

        format!("Expected identifier in line:\n{}", position)
    }
}

#[derive(Debug)]
pub struct ExpectedStringError {
    pub cursor: PageCursor,
}

impl ErrorDisplay for ExpectedStringError {
    fn display(&self, source: Source) -> String {
        let position = source
            .display_position(self.cursor)
            .expect("tokenizer error should point to valid position");

        format!("Expected string in line:\n{}", position)
    }
}
