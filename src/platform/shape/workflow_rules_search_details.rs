#[derive(Clone, Copy, Debug)]
pub struct WorkflowRulesSearchDetails;

impl WorkflowRulesSearchDetails {
    pub const INVALID_RULES: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("invalidRules");

    pub const VALID_RULES: crate::Field<
        Self,
        crate::List<crate::Object<super::WorkflowTransitionRules>>,
    > = crate::Field::new("validRules");

    pub const WORKFLOW_ENTITY_ID: crate::Field<Self, crate::Text> =
        crate::Field::new("workflowEntityId");
}
