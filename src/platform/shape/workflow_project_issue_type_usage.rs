#[derive(Clone, Copy, Debug)]
pub struct WorkflowProjectIssueTypeUsage;

impl WorkflowProjectIssueTypeUsage {
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
}
