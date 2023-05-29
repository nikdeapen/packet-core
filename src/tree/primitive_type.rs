use std::fmt::{Display, Formatter};

/// A primitive type.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum PrimitiveType {
    IntUnsigned8,
    IntUnsigned16,
    IntUnsigned32,
    IntUnsigned64,
    IntUnsigned128,
}

impl PrimitiveType {
    //! Display

    /// Converts the primitive type to a static string.
    pub const fn to_str(&self) -> &'static str {
        match self {
            Self::IntUnsigned8 => "u8",
            Self::IntUnsigned16 => "u16",
            Self::IntUnsigned32 => "u32",
            Self::IntUnsigned64 => "u64",
            Self::IntUnsigned128 => "u128",
        }
    }
}

impl Display for PrimitiveType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_str())
    }
}
