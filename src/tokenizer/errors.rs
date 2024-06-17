use crate::{
    source::Source,
    traits::{error_display::ErrorDisplay, page_position::PageCursor},
};

#[derive(Debug, Clone, PartialEq)]
pub struct UnrecognizedTokenError {
    pub unrecognized_character: char,
    pub cursor: PageCursor,
}

impl ErrorDisplay for UnrecognizedTokenError {
    fn display(&self, source: Source) -> String {
        let position = source
            .display_position(self.cursor)
            .expect("tokenizer error should point to valid position");

        format!(
            "Unrecognized character '{}' found in line:\n{}",
            self.unrecognized_character.escape_debug(),
            position
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnrecognizedKeywordError {
    pub unrecognized_keyword: String,
    pub cursor: PageCursor,
}

impl ErrorDisplay for UnrecognizedKeywordError {
    fn display(&self, source: Source) -> String {
        let position = source
            .display_position(self.cursor)
            .expect("tokenizer error should point to valid position");

        format!(
            "Unrecognized keyword \"{}\" found in line:\n{}",
            self.unrecognized_keyword.escape_debug().collect::<String>(),
            position
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::traits::{error_display::ErrorDisplay, page_position::PageCursor};

    use super::UnrecognizedTokenError;

    #[test]
    fn tokenizer_error_displays() {
        let err = UnrecognizedTokenError {
            unrecognized_character: '|',
            cursor: PageCursor::start(),
        };

        let displayed = err.display("| test\n".into());

        assert_eq!(
            displayed,
            r#"Unrecognized character '|' found in line:
1 | | test
  | ^"#
        )
    }
}
