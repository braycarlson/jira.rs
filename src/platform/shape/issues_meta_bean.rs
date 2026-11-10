#[derive(Clone, Copy, Debug)]
pub struct IssuesMetaBean;

impl IssuesMetaBean {
    pub const JQL: crate::Field<Self, crate::Object<super::IssuesJQLMetaDataBean>> =
        crate::Field::new("jql");
}
