#[derive(Clone, Copy, Debug)]
pub struct StatusProjectIssueTypeUsageDTO;

impl StatusProjectIssueTypeUsageDTO {
    pub const ISSUE_TYPES: crate::Field<
        Self,
        crate::Object<super::StatusProjectIssueTypeUsagePage>,
    > = crate::Field::new("issueTypes");

    pub const PROJECT_ID: crate::Field<Self, crate::Text> = crate::Field::new("projectId");
    pub const STATUS_ID: crate::Field<Self, crate::Text> = crate::Field::new("statusId");
}
