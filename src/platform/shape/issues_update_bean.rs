#[derive(Clone, Copy, Debug)]
pub struct IssuesUpdateBean;

impl IssuesUpdateBean {
    pub const ISSUE_UPDATES: crate::Field<
        Self,
        crate::List<crate::Object<super::IssueUpdateDetails>>,
    > = crate::Field::new("issueUpdates");
}
