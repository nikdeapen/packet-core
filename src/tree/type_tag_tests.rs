use crate::tree::PrimitiveType::IntUnsigned8;
use crate::tree::TypeTag;

#[test]
fn display() {
    assert_eq!(TypeTag::from(IntUnsigned8).to_string(), "u8");
    assert_eq!(TypeTag::from("MyType").to_string(), "MyType");
}
