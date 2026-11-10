#[derive(Clone, Copy, Debug)]
pub struct IssueArchivalSyncRequest;

impl IssueArchivalSyncRequest {
    pub const ISSUE_IDS_OR_KEYS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("issueIdsOrKeys");
}
