#[derive(Clone, Copy, Debug)]
pub struct WorkflowProjectIDScope;

impl WorkflowProjectIDScope {
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
}
