use crate::tree::Variable;

/// A message field.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct MessageField {
    var: Variable,
    tag: Option<u32>,
}

impl<V: Into<Variable>> From<V> for MessageField {
    fn from(var: V) -> Self {
        Self {
            var: var.into(),
            tag: None,
        }
    }
}

impl MessageField {
    //! Mutations

    /// Sets the tag.
    pub fn with_tag(mut self, tag: u32) -> Self {
        self.tag = Some(tag);
        self
    }
}

impl MessageField {
    //! Properties

    /// Gets the variable.
    pub fn var(&self) -> &Variable {
        &self.var
    }

    /// Gets the tag.
    pub fn tag(&self) -> Option<u32> {
        self.tag
    }
}
