#[derive(Clone, Copy, Debug)]
pub struct SubmittedBulkOperation;

impl SubmittedBulkOperation {
    pub const TASK_ID: crate::Field<Self, crate::Text> = crate::Field::new("taskId");
}
