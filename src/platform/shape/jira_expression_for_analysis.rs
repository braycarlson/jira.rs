#[derive(Clone, Copy, Debug)]
pub struct JiraExpressionForAnalysis;

impl JiraExpressionForAnalysis {
    pub const CONTEXT_VARIABLES: crate::Field<Self, crate::Map<crate::Text>> =
        crate::Field::new("contextVariables");

    pub const EXPRESSIONS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("expressions");
}
