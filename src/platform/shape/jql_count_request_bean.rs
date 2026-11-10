#[derive(Clone, Copy, Debug)]
pub struct JQLCountRequestBean;

impl JQLCountRequestBean {
    pub const JQL: crate::Field<Self, crate::Text> = crate::Field::new("jql");
}
