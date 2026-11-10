#[derive(Clone, Copy, Debug)]
pub struct JQLQueryClauseTimePredicate;

impl JQLQueryClauseTimePredicate {
    pub const OPERAND: crate::Field<Self, crate::Any> = crate::Field::new("operand");
    pub const OPERATOR: crate::Field<Self, crate::Text> = crate::Field::new("operator");
}
