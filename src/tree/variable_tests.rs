use crate::tree::PrimitiveType::IntUnsigned8;
use crate::tree::Variable;

#[test]
fn display() {
    let variable: Variable = ("my_var", IntUnsigned8).into();
    assert_eq!(variable.to_string(), "my_var: u8");
}
