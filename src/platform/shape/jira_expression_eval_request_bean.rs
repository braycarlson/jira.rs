#[derive(Clone, Copy, Debug)]
pub struct JiraExpressionEvalRequestBean;

impl JiraExpressionEvalRequestBean {
    pub const CONTEXT: crate::Field<Self, crate::Object<super::JiraExpressionEvalContextBean>> =
        crate::Field::new("context");

    pub const EXPRESSION: crate::Field<Self, crate::Text> = crate::Field::new("expression");
}
