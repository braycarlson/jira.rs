#[derive(Clone, Copy, Debug)]
pub struct WorkflowTransitionRulesDetails;

impl WorkflowTransitionRulesDetails {
    pub const WORKFLOW_ID: crate::Field<Self, crate::Object<super::WorkflowID>> =
        crate::Field::new("workflowId");

    pub const WORKFLOW_RULE_IDS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("workflowRuleIds");
}
