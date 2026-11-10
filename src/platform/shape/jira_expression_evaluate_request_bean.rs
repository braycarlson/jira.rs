#[derive(Clone, Copy, Debug)]
pub struct JiraExpressionEvaluateRequestBean;

impl JiraExpressionEvaluateRequestBean {
    pub const CONTEXT: crate::Field<Self, crate::Object<super::JiraExpressionEvaluateContextBean>> =
        crate::Field::new("context");

    pub const EXPRESSION: crate::Field<Self, crate::Text> = crate::Field::new("expression");
}
