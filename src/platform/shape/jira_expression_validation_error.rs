#[derive(Clone, Copy, Debug)]
pub struct JiraExpressionValidationError;

impl JiraExpressionValidationError {
    pub const COLUMN: crate::Field<Self, crate::Integer> = crate::Field::new("column");
    pub const EXPRESSION: crate::Field<Self, crate::Text> = crate::Field::new("expression");
    pub const LINE: crate::Field<Self, crate::Integer> = crate::Field::new("line");
    pub const MESSAGE: crate::Field<Self, crate::Text> = crate::Field::new("message");
    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
}
