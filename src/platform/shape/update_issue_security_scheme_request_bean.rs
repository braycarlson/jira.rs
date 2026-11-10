#[derive(Clone, Copy, Debug)]
pub struct UpdateIssueSecuritySchemeRequestBean;

impl UpdateIssueSecuritySchemeRequestBean {
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
}
