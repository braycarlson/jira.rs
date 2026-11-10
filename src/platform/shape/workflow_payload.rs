#[derive(Clone, Copy, Debug)]
pub struct WorkflowPayload;

impl WorkflowPayload {
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");

    pub const LOOPED_TRANSITION_CONTAINER_LAYOUT: crate::Field<
        Self,
        crate::Object<super::WorkflowStatusLayoutPayload>,
    > = crate::Field::new("loopedTransitionContainerLayout");

    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
    pub const ON_CONFLICT: crate::Field<Self, crate::Text> = crate::Field::new("onConflict");

    pub const PCRI: crate::Field<Self, crate::Object<super::ProjectCreateResourceIdentifier>> =
        crate::Field::new("pcri");

    pub const START_POINT_LAYOUT: crate::Field<
        Self,
        crate::Object<super::WorkflowStatusLayoutPayload>,
    > = crate::Field::new("startPointLayout");

    pub const STATUSES: crate::Field<
        Self,
        crate::List<crate::Object<super::WorkflowStatusPayload>>,
    > = crate::Field::new("statuses");

    pub const TRANSITIONS: crate::Field<
        Self,
        crate::List<crate::Object<super::TransitionPayload>>,
    > = crate::Field::new("transitions");
}
