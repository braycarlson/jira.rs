#[derive(Clone, Copy, Debug)]
pub struct ValueOperand;

impl ValueOperand {
    pub const ENCODED_VALUE: crate::Field<Self, crate::Text> = crate::Field::new("encodedValue");
    pub const VALUE: crate::Field<Self, crate::Text> = crate::Field::new("value");
}
