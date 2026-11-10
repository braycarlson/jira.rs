#[derive(Clone, Copy, Debug)]
pub struct FieldValueClause;

impl FieldValueClause {
    pub const FIELD: crate::Field<Self, crate::Object<super::JQLQueryField>> =
        crate::Field::new("field");

    pub const OPERAND: crate::Field<Self, crate::Any> = crate::Field::new("operand");
    pub const OPERATOR: crate::Field<Self, crate::Text> = crate::Field::new("operator");
}
