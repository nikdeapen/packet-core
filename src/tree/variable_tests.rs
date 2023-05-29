use crate::tree::PrimitiveType::IntUnsigned8;
use crate::tree::Variable;

#[test]
fn properties() {
    let var: Variable = ("my_var", IntUnsigned8).into();
    assert_eq!(var.name(), "my_var");
    assert_eq!(var.tag(), &IntUnsigned8.into());
}
