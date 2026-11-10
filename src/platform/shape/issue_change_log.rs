#[derive(Clone, Copy, Debug)]
pub struct IssueChangeLog;

impl IssueChangeLog {
    pub const CHANGE_HISTORIES: crate::Field<Self, crate::List<crate::Object<super::Changelog>>> =
        crate::Field::new("changeHistories");

    pub const ISSUE_ID: crate::Field<Self, crate::Text> = crate::Field::new("issueId");
}
