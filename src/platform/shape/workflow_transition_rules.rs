#[derive(Clone, Copy, Debug)]
pub struct WorkflowTransitionRules;

impl WorkflowTransitionRules {
    pub const CONDITIONS: crate::Field<
        Self,
        crate::List<crate::Object<super::AppWorkflowTransitionRule>>,
    > = crate::Field::new("conditions");

    pub const POST_FUNCTIONS: crate::Field<
        Self,
        crate::List<crate::Object<super::AppWorkflowTransitionRule>>,
    > = crate::Field::new("postFunctions");

    pub const VALIDATORS: crate::Field<
        Self,
        crate::List<crate::Object<super::AppWorkflowTransitionRule>>,
    > = crate::Field::new("validators");

    pub const WORKFLOW_ID: crate::Field<Self, crate::Object<super::WorkflowID>> =
        crate::Field::new("workflowId");
}
