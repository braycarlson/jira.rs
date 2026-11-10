#[derive(Clone, Copy, Debug)]
pub struct IssueSecuritySchemeToProjectMapping;

impl IssueSecuritySchemeToProjectMapping {
    pub const ISSUE_SECURITY_SCHEME_ID: crate::Field<Self, crate::Text> =
        crate::Field::new("issueSecuritySchemeId");

    pub const PROJECT_ID: crate::Field<Self, crate::Text> = crate::Field::new("projectId");
}
