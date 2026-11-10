#[derive(Clone, Copy, Debug)]
pub struct IssueTypeSchemeProjects;

impl IssueTypeSchemeProjects {
    pub const ISSUE_TYPE_SCHEME: crate::Field<Self, crate::Object<super::IssueTypeScheme>> =
        crate::Field::new("issueTypeScheme");

    pub const PROJECT_IDS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("projectIds");
}
