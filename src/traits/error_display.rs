use crate::source::Source;

pub trait ErrorDisplay {
    fn display(&self, source: Source) -> String;
}
