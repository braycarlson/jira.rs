#[derive(Clone, Copy, Debug)]
pub struct AvailableWorkflowTriggerTypes;

impl AvailableWorkflowTriggerTypes {
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
}
