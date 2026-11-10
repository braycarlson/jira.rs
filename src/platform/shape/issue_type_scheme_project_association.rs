#[derive(Clone, Copy, Debug)]
pub struct IssueTypeSchemeProjectAssociation;

impl IssueTypeSchemeProjectAssociation {
    pub const ISSUE_TYPE_SCHEME_ID: crate::Field<Self, crate::Text> =
        crate::Field::new("issueTypeSchemeId");

    pub const PROJECT_ID: crate::Field<Self, crate::Text> = crate::Field::new("projectId");
}
