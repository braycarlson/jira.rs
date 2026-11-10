#[derive(Clone, Copy, Debug)]
pub struct WorkflowCompoundCondition;

impl WorkflowCompoundCondition {
    pub const CONDITIONS: crate::Field<Self, crate::List<crate::Any>> =
        crate::Field::new("conditions");

    pub const NODE_TYPE: crate::Field<Self, crate::Text> = crate::Field::new("nodeType");
    pub const OPERATOR: crate::Field<Self, crate::Text> = crate::Field::new("operator");
}
