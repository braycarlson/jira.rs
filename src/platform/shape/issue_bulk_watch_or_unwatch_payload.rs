#[derive(Clone, Copy, Debug)]
pub struct IssueBulkWatchOrUnwatchPayload;

impl IssueBulkWatchOrUnwatchPayload {
    pub const SELECTED_ISSUE_IDS_OR_KEYS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("selectedIssueIdsOrKeys");
}
