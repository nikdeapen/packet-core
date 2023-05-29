use crate::tree::PrimitiveType::{
    IntUnsigned128, IntUnsigned16, IntUnsigned32, IntUnsigned64, IntUnsigned8,
};

#[test]
fn display() {
    assert_eq!(IntUnsigned8.to_string(), "u8");
    assert_eq!(IntUnsigned16.to_string(), "u16");
    assert_eq!(IntUnsigned32.to_string(), "u32");
    assert_eq!(IntUnsigned64.to_string(), "u64");
    assert_eq!(IntUnsigned128.to_string(), "u128");
}
