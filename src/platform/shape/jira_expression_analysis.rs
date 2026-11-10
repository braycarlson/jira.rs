#[derive(Clone, Copy, Debug)]
pub struct JiraExpressionAnalysis;

impl JiraExpressionAnalysis {
    pub const COMPLEXITY: crate::Field<Self, crate::Object<super::JiraExpressionComplexity>> =
        crate::Field::new("complexity");

    pub const ERRORS: crate::Field<
        Self,
        crate::List<crate::Object<super::JiraExpressionValidationError>>,
    > = crate::Field::new("errors");

    pub const EXPRESSION: crate::Field<Self, crate::Text> = crate::Field::new("expression");
    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
    pub const VALID: crate::Field<Self, crate::Boolean> = crate::Field::new("valid");
}
