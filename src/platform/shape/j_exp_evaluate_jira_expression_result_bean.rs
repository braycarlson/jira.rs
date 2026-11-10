#[derive(Clone, Copy, Debug)]
pub struct JExpEvaluateJiraExpressionResultBean;

impl JExpEvaluateJiraExpressionResultBean {
    pub const META: crate::Field<Self, crate::Object<super::JExpEvaluateMetaDataBean>> =
        crate::Field::new("meta");

    pub const VALUE: crate::Field<Self, crate::Any> = crate::Field::new("value");
}
