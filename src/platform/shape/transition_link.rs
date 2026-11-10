#[derive(Clone, Copy, Debug)]
pub struct TransitionLink;

impl TransitionLink {
    pub const FROM_PORT: crate::Field<Self, crate::Integer> = crate::Field::new("fromPort");

    pub const FROM_STATUS_REFERENCE: crate::Field<Self, crate::Text> =
        crate::Field::new("fromStatusReference");

    pub const TO_PORT: crate::Field<Self, crate::Integer> = crate::Field::new("toPort");
}
