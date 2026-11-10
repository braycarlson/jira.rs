#[derive(Clone, Copy, Debug)]
pub struct JiraExpressionComplexity;

impl JiraExpressionComplexity {
    pub const EXPENSIVE_OPERATIONS: crate::Field<Self, crate::Text> =
        crate::Field::new("expensiveOperations");

    pub const VARIABLES: crate::Field<Self, crate::Map<crate::Text>> =
        crate::Field::new("variables");
}
