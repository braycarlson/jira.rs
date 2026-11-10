#[derive(Clone, Copy, Debug)]
pub struct BulkTransitionGetAvailableTransitions;

impl BulkTransitionGetAvailableTransitions {
    pub const AVAILABLE_TRANSITIONS: crate::Field<
        Self,
        crate::List<crate::Object<super::IssueBulkTransitionForWorkflow>>,
    > = crate::Field::new("availableTransitions");

    pub const ENDING_BEFORE: crate::Field<Self, crate::Text> = crate::Field::new("endingBefore");
    pub const STARTING_AFTER: crate::Field<Self, crate::Text> = crate::Field::new("startingAfter");
}
