use crate::tree::MessageField;

/// A message type.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Message {
    name: String,
    fields: Vec<MessageField>,
}

impl<S: Into<String>> From<S> for Message {
    fn from(name: S) -> Self {
        Self {
            name: name.into(),
            fields: Vec::default(),
        }
    }
}

impl Message {
    //! Mutations

    /// Adds the field.
    pub fn with_field<F>(mut self, field: F) -> Self
    where
        F: Into<MessageField>,
    {
        self.fields.push(field.into());
        self
    }
}

impl Message {
    //! Properties

    /// Gets the name.
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Gets the fields.
    pub fn fields(&self) -> &[MessageField] {
        self.fields.as_slice()
    }
}
