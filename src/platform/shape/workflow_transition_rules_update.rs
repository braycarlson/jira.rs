#[derive(Clone, Copy, Debug)]
pub struct WorkflowTransitionRulesUpdate;

impl WorkflowTransitionRulesUpdate {
    pub const WORKFLOWS: crate::Field<
        Self,
        crate::List<crate::Object<super::WorkflowTransitionRules>>,
    > = crate::Field::new("workflows");
}
