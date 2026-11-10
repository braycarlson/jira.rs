#[derive(Clone, Copy, Debug)]
pub struct WorkflowID;

impl WorkflowID {
    pub const DRAFT: crate::Field<Self, crate::Boolean> = crate::Field::new("draft");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
}
