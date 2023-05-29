use std::fmt::{Display, Formatter};

use crate::tree::TypeTag;

/// A name with an associated type tag.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Variable {
    name: String,
    tag: TypeTag,
}

impl<S: Into<String>, T: Into<TypeTag>> From<(S, T)> for Variable {
    fn from(t: (S, T)) -> Self {
        Self {
            name: t.0.into(),
            tag: t.1.into(),
        }
    }
}

impl Display for Variable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.name, self.tag)
    }
}
