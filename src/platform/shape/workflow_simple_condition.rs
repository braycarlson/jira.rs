#[derive(Clone, Copy, Debug)]
pub struct WorkflowSimpleCondition;

impl WorkflowSimpleCondition {
    pub const CONFIGURATION: crate::Field<Self, crate::Any> = crate::Field::new("configuration");
    pub const NODE_TYPE: crate::Field<Self, crate::Text> = crate::Field::new("nodeType");
    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
}
