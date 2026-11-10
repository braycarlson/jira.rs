#[derive(Clone, Copy, Debug)]
pub struct WorkflowSchemeUsage;

impl WorkflowSchemeUsage {
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
}
