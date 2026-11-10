#[derive(Clone, Copy, Debug)]
pub struct JiraExpressionEvaluationMetaDataBean;

impl JiraExpressionEvaluationMetaDataBean {
    pub const COMPLEXITY: crate::Field<Self, crate::Object<super::JiraExpressionsComplexityBean>> =
        crate::Field::new("complexity");

    pub const ISSUES: crate::Field<Self, crate::Object<super::IssuesMetaBean>> =
        crate::Field::new("issues");
}
