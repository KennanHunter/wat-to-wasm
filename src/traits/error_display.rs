use std::fmt::Debug;

use crate::source::Source;

pub trait ErrorDisplay: Debug {
    fn display(&self, source: Source) -> String;
}
