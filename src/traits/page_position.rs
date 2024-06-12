pub trait PagePosition {
    fn position(&self) -> PageCursor;
}

#[derive(Debug, Clone, Copy)]
pub struct PageCursor {
    pub line: usize,
    pub column: usize,
}

/**
 * Describes a location in the source code, where the
 */
impl PageCursor {
    pub fn start() -> PageCursor {
        PageCursor { line: 1, column: 0 }
    }

    pub fn advance_line(&self) -> PageCursor {
        Self {
            line: self.line + 1,
            column: 0,
        }
    }

    pub fn advance_column(&self) -> PageCursor {
        Self {
            line: self.line,
            column: self.column + 1,
        }
    }
}
