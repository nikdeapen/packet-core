use crate::tree::PrimitiveType::IntUnsigned8;
use crate::tree::{MessageField, Variable};

#[test]
fn properties() {
    let field: MessageField = ("one", IntUnsigned8).into();
    assert_eq!(field.var(), &Variable::from(("one", IntUnsigned8)));
    assert_eq!(field.tag(), None);

    let field: MessageField = field.with_tag(1);
    assert_eq!(field.tag(), Some(1));
}
