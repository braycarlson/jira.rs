#[derive(Clone, Copy, Debug)]
pub struct ArchiveIssueAsyncRequest;

impl ArchiveIssueAsyncRequest {
    pub const JQL: crate::Field<Self, crate::Text> = crate::Field::new("jql");
}
