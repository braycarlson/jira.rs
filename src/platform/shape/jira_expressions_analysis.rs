#[derive(Clone, Copy, Debug)]
pub struct JiraExpressionsAnalysis;

impl JiraExpressionsAnalysis {
    pub const RESULTS: crate::Field<
        Self,
        crate::List<crate::Object<super::JiraExpressionAnalysis>>,
    > = crate::Field::new("results");
}
