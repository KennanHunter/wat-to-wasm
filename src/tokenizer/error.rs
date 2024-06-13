use crate::{source::Source, traits::page_position::PageCursor};

#[derive(Debug, Clone, PartialEq)]
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
            "Unrecognized character '{}' found in line:\n{}",
            self.unrecognized_character.escape_debug(),
            position
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::traits::page_position::PageCursor;

    use super::TokenizerError;

    #[test]
    fn tokenizer_error_displays() {
        let err = TokenizerError {
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
