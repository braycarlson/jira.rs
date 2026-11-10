#[derive(Clone, Copy, Debug)]
pub struct IssueBulkDeletePayload;

impl IssueBulkDeletePayload {
    pub const SELECTED_ISSUE_IDS_OR_KEYS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("selectedIssueIdsOrKeys");

    pub const SEND_BULK_NOTIFICATION: crate::Field<Self, crate::Boolean> =
        crate::Field::new("sendBulkNotification");
}
