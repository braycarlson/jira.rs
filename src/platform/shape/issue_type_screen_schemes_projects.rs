#[derive(Clone, Copy, Debug)]
pub struct IssueTypeScreenSchemesProjects;

impl IssueTypeScreenSchemesProjects {
    pub const ISSUE_TYPE_SCREEN_SCHEME: crate::Field<
        Self,
        crate::Object<super::IssueTypeScreenScheme>,
    > = crate::Field::new("issueTypeScreenScheme");

    pub const PROJECT_IDS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("projectIds");
}
