use crate::tree::PrimitiveType::{IntUnsigned16, IntUnsigned8};
use crate::tree::{Message, MessageField};

#[test]
fn properties() {
    let message: Message = "MyMessage".into();
    assert_eq!(message.name(), "MyMessage");

    let one: MessageField = MessageField::from(("one", IntUnsigned8)).with_tag(1);
    let two: MessageField = MessageField::from(("two", IntUnsigned16)).with_tag(2);
    let message: Message = message.with_field(one.clone()).with_field(two.clone());
    assert_eq!(message.fields(), &[one, two]);
}
