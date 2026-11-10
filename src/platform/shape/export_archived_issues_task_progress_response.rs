#[derive(Clone, Copy, Debug)]
pub struct ExportArchivedIssuesTaskProgressResponse;

impl ExportArchivedIssuesTaskProgressResponse {
    pub const FILE_URL: crate::Field<Self, crate::Text> = crate::Field::new("fileUrl");
    pub const PAYLOAD: crate::Field<Self, crate::Text> = crate::Field::new("payload");
    pub const PROGRESS: crate::Field<Self, crate::Integer> = crate::Field::new("progress");
    pub const STATUS: crate::Field<Self, crate::Text> = crate::Field::new("status");

    pub const SUBMITTED_TIME: crate::Field<Self, crate::Integer> =
        crate::Field::new("submittedTime");

    pub const TASK_ID: crate::Field<Self, crate::Text> = crate::Field::new("taskId");
}
