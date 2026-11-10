#[derive(Clone, Copy, Debug)]
pub struct WorkflowCreate;

impl WorkflowCreate {
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");

    pub const LOOPED_TRANSITION_CONTAINER_LAYOUT: crate::Field<
        Self,
        crate::Object<super::WorkflowLayout>,
    > = crate::Field::new("loopedTransitionContainerLayout");

    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");

    pub const START_POINT_LAYOUT: crate::Field<Self, crate::Object<super::WorkflowLayout>> =
        crate::Field::new("startPointLayout");

    pub const STATUSES: crate::Field<Self, crate::List<crate::Object<super::StatusLayoutUpdate>>> =
        crate::Field::new("statuses");

    pub const TRANSITIONS: crate::Field<
        Self,
        crate::List<crate::Object<super::TransitionUpdateDTO>>,
    > = crate::Field::new("transitions");
}
