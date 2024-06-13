use std::rc::Rc;

use crate::traits::page_position::PageCursor;

#[derive(Debug, Clone)]
pub struct Source {
    lines: Rc<Vec<String>>,
}

impl From<String> for Source {
    fn from(value: String) -> Self {
        Self::from(value.as_str())
    }
}

impl From<&str> for Source {
    fn from(value: &str) -> Self {
        let lines = value.split_inclusive('\n').map(|line| line.to_owned());

        Self {
            lines: Rc::new(lines.collect()),
        }
    }
}

impl Source {
    pub fn at_position(&self, position: PageCursor) -> Option<char> {
        match (*self.lines).get(position.line - 1) {
            Some(line) => match line.chars().nth(position.column) {
                Some(char) => Some(char),
                None => None,
            },
            None => None,
        }
    }

    pub fn display_position(&self, position: PageCursor) -> Result<String, PageCursor> {
        let mut res = String::new();

        let line = if let Some(line) = self.lines.get(position.line - 1) {
            position.line.to_string() + " | " + line.trim()
        } else {
            return Err(position);
        };

        res.push_str(&line);

        res.push('\n');

        for _ in 0..position.line.to_string().len() {
            res.push(' ')
        }

        res.push_str(" | ");

        for _ in 0..(position.column) {
            res.push(' ');
        }

        res.push('^');

        Ok(res.to_string())
    }
}

impl IntoIterator for Source {
    type Item = (char, PageCursor);
    type IntoIter = SourceIter;

    fn into_iter(self) -> Self::IntoIter {
        SourceIter::from(self.clone())
    }
}

#[derive(Debug)]
pub struct SourceIter {
    current_cursor: Option<PageCursor>,
    source: Source,
}

impl SourceIter {
    pub fn next_if_char(&mut self, ch: char) -> Option<(char, PageCursor)> {
        let cursor = match self.current_cursor {
            Some(cursor) => cursor,
            None => {
                let cursor = PageCursor::start();

                return Some((self.source.at_position(PageCursor::start())?, cursor));
            }
        };

        let next_position_in_line = cursor.advance_column();

        if let Some(char) = self.source.at_position(next_position_in_line) {
            if char == ch {
                self.current_cursor = Some(next_position_in_line);
                return Some((char, next_position_in_line));
            }
        }

        let start_of_next_line = cursor.advance_line();

        if let Some(char) = self.source.at_position(start_of_next_line) {
            if char == ch {
                self.current_cursor = Some(start_of_next_line);
                return Some((char, start_of_next_line));
            }
        }

        None
    }
}

impl From<Source> for SourceIter {
    fn from(value: Source) -> Self {
        Self {
            current_cursor: None,
            source: value.clone(),
        }
    }
}

impl Iterator for SourceIter {
    type Item = (char, PageCursor);

    fn next(&mut self) -> Option<Self::Item> {
        let cursor = match self.current_cursor {
            Some(cursor) => cursor,
            None => {
                let cursor = PageCursor::start();

                self.current_cursor = Some(cursor);

                return Some((self.source.at_position(cursor)?, cursor));
            }
        };

        let next_position_in_line = cursor.advance_column();

        if let Some(char) = self.source.at_position(next_position_in_line) {
            self.current_cursor = Some(next_position_in_line);
            return Some((char, next_position_in_line));
        }

        let start_of_next_line = cursor.advance_line();

        if let Some(char) = self.source.at_position(start_of_next_line) {
            self.current_cursor = Some(start_of_next_line);
            return Some((char, start_of_next_line));
        }

        None
    }
}
