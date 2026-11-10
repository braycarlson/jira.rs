#[derive(Clone, Copy, Debug)]
pub struct FunctionOperand;

impl FunctionOperand {
    pub const ARGUMENTS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("arguments");

    pub const ENCODED_OPERAND: crate::Field<Self, crate::Text> =
        crate::Field::new("encodedOperand");

    pub const FUNCTION: crate::Field<Self, crate::Text> = crate::Field::new("function");
}
