#[derive(Clone, Copy, Debug)]
pub struct WorkflowTransitionRulesUpdateErrors;

impl WorkflowTransitionRulesUpdateErrors {
    pub const UPDATE_RESULTS: crate::Field<
        Self,
        crate::List<crate::Object<super::WorkflowTransitionRulesUpdateErrorDetails>>,
    > = crate::Field::new("updateResults");
}
