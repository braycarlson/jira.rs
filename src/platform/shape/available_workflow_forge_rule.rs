#[derive(Clone, Copy, Debug)]
pub struct AvailableWorkflowForgeRule;

impl AvailableWorkflowForgeRule {
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
    pub const RULE_KEY: crate::Field<Self, crate::Text> = crate::Field::new("ruleKey");
    pub const RULE_TYPE: crate::Field<Self, crate::Text> = crate::Field::new("ruleType");
}
