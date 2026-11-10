#[derive(Clone, Copy, Debug)]
pub struct JiraExpressionResult;

impl JiraExpressionResult {
    pub const META: crate::Field<Self, crate::Object<super::JiraExpressionEvaluationMetaDataBean>> =
        crate::Field::new("meta");

    pub const VALUE: crate::Field<Self, crate::Any> = crate::Field::new("value");
}
