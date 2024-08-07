#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Identifier(pub String);

impl From<String> for Identifier {
    fn from(value: String) -> Self {
        Self(value)
    }
}
