use crate::{
    source::Source,
    traits::{error_display::ErrorDisplay, page_position::PageCursor},
};

#[derive(Debug)]
pub struct MisplacedError {
    pub cursor: PageCursor,
}

impl ErrorDisplay for MisplacedError {
    fn display(&self, source: Source) -> String {
        let position = source
            .display_position(self.cursor)
            .expect("parser error should point to valid position");

        format!("Misplaced identifier in:\n{}", position)
    }
}
