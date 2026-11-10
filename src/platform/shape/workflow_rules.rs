#[derive(Clone, Copy, Debug)]
pub struct WorkflowRules;

impl WorkflowRules {
    pub const CONDITIONS_TREE: crate::Field<Self, crate::Any> = crate::Field::new("conditionsTree");

    pub const POST_FUNCTIONS: crate::Field<
        Self,
        crate::List<crate::Object<super::WorkflowTransitionRule>>,
    > = crate::Field::new("postFunctions");

    pub const VALIDATORS: crate::Field<
        Self,
        crate::List<crate::Object<super::WorkflowTransitionRule>>,
    > = crate::Field::new("validators");
}
