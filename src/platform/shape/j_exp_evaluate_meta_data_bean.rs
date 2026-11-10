#[derive(Clone, Copy, Debug)]
pub struct JExpEvaluateMetaDataBean;

impl JExpEvaluateMetaDataBean {
    pub const COMPLEXITY: crate::Field<Self, crate::Object<super::JiraExpressionsComplexityBean>> =
        crate::Field::new("complexity");

    pub const ISSUES: crate::Field<Self, crate::Object<super::JExpEvaluateIssuesMetaBean>> =
        crate::Field::new("issues");
}
