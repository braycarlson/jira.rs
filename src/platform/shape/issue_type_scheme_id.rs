#[derive(Clone, Copy, Debug)]
pub struct IssueTypeSchemeID;

impl IssueTypeSchemeID {
    pub const ISSUE_TYPE_SCHEME_ID: crate::Field<Self, crate::Text> =
        crate::Field::new("issueTypeSchemeId");
}
