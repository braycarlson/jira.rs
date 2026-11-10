#[derive(Clone, Copy, Debug)]
pub struct Fields;

impl Fields {
    pub const ASSIGNEE: crate::Field<Self, crate::Object<super::UserDetails>> =
        crate::Field::new("assignee");

    pub const ISSUETYPE: crate::Field<Self, crate::Object<super::IssueTypeDetails>> =
        crate::Field::new("issuetype");

    pub const ISSUE_TYPE: crate::Field<Self, crate::Object<super::IssueTypeDetails>> =
        crate::Field::new("issueType");

    pub const PRIORITY: crate::Field<Self, crate::Object<super::Priority>> =
        crate::Field::new("priority");

    pub const STATUS: crate::Field<Self, crate::Object<super::StatusDetails>> =
        crate::Field::new("status");

    pub const SUMMARY: crate::Field<Self, crate::Text> = crate::Field::new("summary");

    pub const TIMETRACKING: crate::Field<Self, crate::Object<super::TimeTrackingDetails>> =
        crate::Field::new("timetracking");
}
