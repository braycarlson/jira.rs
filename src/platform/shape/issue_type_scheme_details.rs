#[derive(Clone, Copy, Debug)]
pub struct IssueTypeSchemeDetails;

impl IssueTypeSchemeDetails {
    pub const DEFAULT_ISSUE_TYPE_ID: crate::Field<Self, crate::Text> =
        crate::Field::new("defaultIssueTypeId");

    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");

    pub const ISSUE_TYPE_IDS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("issueTypeIds");

    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
}
