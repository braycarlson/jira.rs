#[derive(Clone, Copy, Debug)]
pub struct RedactionPosition;

impl RedactionPosition {
    pub const ADF_POINTER: crate::Field<Self, crate::Text> = crate::Field::new("adfPointer");
    pub const EXPECTED_TEXT: crate::Field<Self, crate::Text> = crate::Field::new("expectedText");
    pub const FROM: crate::Field<Self, crate::Integer> = crate::Field::new("from");
    pub const TO_VALUE: crate::Field<Self, crate::Integer> = crate::Field::new("to");
}
