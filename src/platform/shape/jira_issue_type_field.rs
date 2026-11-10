#[derive(Clone, Copy, Debug)]
pub struct JiraIssueTypeField;

impl JiraIssueTypeField {
    pub const ISSUE_TYPE_ID: crate::Field<Self, crate::Text> = crate::Field::new("issueTypeId");
}
