#[derive(Clone, Copy, Debug)]
pub struct IssueBulkEditPayload;

impl IssueBulkEditPayload {
    pub const EDITED_FIELDS_INPUT: crate::Field<Self, crate::Object<super::JiraIssueFields>> =
        crate::Field::new("editedFieldsInput");

    pub const SELECTED_ACTIONS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("selectedActions");

    pub const SELECTED_ISSUE_IDS_OR_KEYS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("selectedIssueIdsOrKeys");

    pub const SEND_BULK_NOTIFICATION: crate::Field<Self, crate::Boolean> =
        crate::Field::new("sendBulkNotification");
}
