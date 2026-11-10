#[derive(Clone, Copy, Debug)]
pub struct WorkflowsWithTransitionRulesDetails;

impl WorkflowsWithTransitionRulesDetails {
    pub const WORKFLOWS: crate::Field<
        Self,
        crate::List<crate::Object<super::WorkflowTransitionRulesDetails>>,
    > = crate::Field::new("workflows");
}
