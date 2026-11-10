#[derive(Clone, Copy, Debug)]
pub struct JExpEvaluateIssuesMetaBean;

impl JExpEvaluateIssuesMetaBean {
    pub const JQL: crate::Field<Self, crate::Object<super::JExpEvaluateIssuesJQLMetaDataBean>> =
        crate::Field::new("jql");
}
