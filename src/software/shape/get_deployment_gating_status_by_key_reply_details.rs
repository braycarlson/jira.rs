#[derive(Clone, Copy, Debug)]
pub struct GetDeploymentGatingStatusByKeyReplyDetails;

impl GetDeploymentGatingStatusByKeyReplyDetails {
    pub const ISSUE_KEY: crate::Field<Self, crate::Text> = crate::Field::new("issueKey");
    pub const ISSUE_LINK: crate::Field<Self, crate::Text> = crate::Field::new("issueLink");
    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
}
