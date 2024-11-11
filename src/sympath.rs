#![allow(dead_code)]

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Hash)]
pub struct SymPath {
    name: String,
}

impl SymPath {
    pub fn from_str(str: &str) -> Self {
        Self {
            name: str.to_string(),
        }
    }

    pub(crate) fn to_string(&self) -> String {
        self.name.to_string()
    }
}
