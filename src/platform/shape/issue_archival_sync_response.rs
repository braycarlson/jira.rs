#[derive(Clone, Copy, Debug)]
pub struct IssueArchivalSyncResponse;

impl IssueArchivalSyncResponse {
    pub const ERRORS: crate::Field<Self, crate::Object<super::Errors>> =
        crate::Field::new("errors");

    pub const NUMBER_OF_ISSUES_UPDATED: crate::Field<Self, crate::Integer> =
        crate::Field::new("numberOfIssuesUpdated");
}
