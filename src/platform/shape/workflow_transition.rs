#[derive(Clone, Copy, Debug)]
pub struct WorkflowTransition;

impl WorkflowTransition {
    pub const ID: crate::Field<Self, crate::Integer> = crate::Field::new("id");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
}
