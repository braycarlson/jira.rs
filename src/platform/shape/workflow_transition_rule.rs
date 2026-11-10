#[derive(Clone, Copy, Debug)]
pub struct WorkflowTransitionRule;

impl WorkflowTransitionRule {
    pub const CONFIGURATION: crate::Field<Self, crate::Any> = crate::Field::new("configuration");
    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
}
