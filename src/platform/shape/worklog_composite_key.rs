#[derive(Clone, Copy, Debug)]
pub struct WorklogCompositeKey;

impl WorklogCompositeKey {
    pub const ISSUE_ID: crate::Field<Self, crate::Integer> = crate::Field::new("issueId");
    pub const WORKLOG_ID: crate::Field<Self, crate::Integer> = crate::Field::new("worklogId");
}
