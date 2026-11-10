#[derive(Clone, Copy, Debug)]
pub struct IssueTypeScreenSchemeProjectAssociation;

impl IssueTypeScreenSchemeProjectAssociation {
    pub const ISSUE_TYPE_SCREEN_SCHEME_ID: crate::Field<Self, crate::Text> =
        crate::Field::new("issueTypeScreenSchemeId");

    pub const PROJECT_ID: crate::Field<Self, crate::Text> = crate::Field::new("projectId");
}
