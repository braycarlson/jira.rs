#[derive(Clone, Copy, Debug)]
pub struct UpdateIssueSecurityLevelDetails;

impl UpdateIssueSecurityLevelDetails {
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
}
