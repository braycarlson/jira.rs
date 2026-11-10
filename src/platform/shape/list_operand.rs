#[derive(Clone, Copy, Debug)]
pub struct ListOperand;

impl ListOperand {
    pub const ENCODED_OPERAND: crate::Field<Self, crate::Text> =
        crate::Field::new("encodedOperand");

    pub const VALUES: crate::Field<Self, crate::List<crate::Any>> = crate::Field::new("values");
}
