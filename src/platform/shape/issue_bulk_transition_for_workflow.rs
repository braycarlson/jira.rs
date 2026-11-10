#[derive(Clone, Copy, Debug)]
pub struct IssueBulkTransitionForWorkflow;

impl IssueBulkTransitionForWorkflow {
    pub const ISSUES: crate::Field<Self, crate::List<crate::Text>> = crate::Field::new("issues");

    pub const IS_TRANSITIONS_FILTERED: crate::Field<Self, crate::Boolean> =
        crate::Field::new("isTransitionsFiltered");

    pub const TRANSITIONS: crate::Field<
        Self,
        crate::List<crate::Object<super::SimplifiedIssueTransition>>,
    > = crate::Field::new("transitions");
}
