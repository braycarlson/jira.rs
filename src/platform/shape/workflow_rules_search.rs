#[derive(Clone, Copy, Debug)]
pub struct WorkflowRulesSearch;

impl WorkflowRulesSearch {
    pub const EXPAND: crate::Field<Self, crate::Text> = crate::Field::new("expand");
    pub const RULE_IDS: crate::Field<Self, crate::List<crate::Text>> = crate::Field::new("ruleIds");

    pub const WORKFLOW_ENTITY_ID: crate::Field<Self, crate::Text> =
        crate::Field::new("workflowEntityId");
}
