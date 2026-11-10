#[derive(Clone, Copy, Debug)]
pub struct IssueTypeSchemeUpdateDetails;

impl IssueTypeSchemeUpdateDetails {
    pub const DEFAULT_ISSUE_TYPE_ID: crate::Field<Self, crate::Text> =
        crate::Field::new("defaultIssueTypeId");

    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
}
