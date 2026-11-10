#[derive(Clone, Copy, Debug)]
pub struct WorkflowTransitionRulesUpdateErrorDetails;

impl WorkflowTransitionRulesUpdateErrorDetails {
    pub const RULE_UPDATE_ERRORS: crate::Field<Self, crate::Map<crate::List<crate::Text>>> =
        crate::Field::new("ruleUpdateErrors");

    pub const UPDATE_ERRORS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("updateErrors");

    pub const WORKFLOW_ID: crate::Field<Self, crate::Object<super::WorkflowID>> =
        crate::Field::new("workflowId");
}
