#[derive(Clone, Copy, Debug)]
pub struct BulkOperationProgress;

impl BulkOperationProgress {
    pub const CREATED: crate::Field<Self, crate::Integer> = crate::Field::new("created");

    pub const FAILED_ACCESSIBLE_ISSUES: crate::Field<Self, crate::Map<crate::List<crate::Text>>> =
        crate::Field::new("failedAccessibleIssues");

    pub const INVALID_OR_INACCESSIBLE_ISSUE_COUNT: crate::Field<Self, crate::Integer> =
        crate::Field::new("invalidOrInaccessibleIssueCount");

    pub const ISSUE_COUNT_TOTAL: crate::Field<Self, crate::Integer> =
        crate::Field::new("totalIssueCount");

    pub const PROCESSED_ACCESSIBLE_ISSUES: crate::Field<Self, crate::List<crate::Integer>> =
        crate::Field::new("processedAccessibleIssues");

    pub const PROGRESS_PERCENT: crate::Field<Self, crate::Integer> =
        crate::Field::new("progressPercent");

    pub const STARTED: crate::Field<Self, crate::Integer> = crate::Field::new("started");
    pub const STATUS: crate::Field<Self, crate::Text> = crate::Field::new("status");

    pub const SUBMITTED_BY: crate::Field<Self, crate::Object<super::User>> =
        crate::Field::new("submittedBy");

    pub const TASK_ID: crate::Field<Self, crate::Text> = crate::Field::new("taskId");
    pub const UPDATED: crate::Field<Self, crate::Integer> = crate::Field::new("updated");
}
