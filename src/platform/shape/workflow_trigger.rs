#[derive(Clone, Copy, Debug)]
pub struct WorkflowTrigger;

impl WorkflowTrigger {
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");

    pub const PARAMETERS: crate::Field<Self, crate::Map<crate::Text>> =
        crate::Field::new("parameters");

    pub const RULE_KEY: crate::Field<Self, crate::Text> = crate::Field::new("ruleKey");
}
