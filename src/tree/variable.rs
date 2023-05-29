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

impl Variable {
    //! Properties

    /// Gets the name.
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Gets the type tag.
    pub fn tag(&self) -> &TypeTag {
        &self.tag
    }
}
