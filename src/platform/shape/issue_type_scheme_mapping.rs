#[derive(Clone, Copy, Debug)]
pub struct IssueTypeSchemeMapping;

impl IssueTypeSchemeMapping {
    pub const ISSUE_TYPE_ID: crate::Field<Self, crate::Text> = crate::Field::new("issueTypeId");

    pub const ISSUE_TYPE_SCHEME_ID: crate::Field<Self, crate::Text> =
        crate::Field::new("issueTypeSchemeId");
}
