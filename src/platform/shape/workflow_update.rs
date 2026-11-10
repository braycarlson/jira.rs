#[derive(Clone, Copy, Debug)]
pub struct WorkflowUpdate;

impl WorkflowUpdate {
    pub const DEFAULT_STATUS_MAPPINGS: crate::Field<
        Self,
        crate::List<crate::Object<super::StatusMigration>>,
    > = crate::Field::new("defaultStatusMappings");

    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");

    pub const LOOPED_TRANSITION_CONTAINER_LAYOUT: crate::Field<
        Self,
        crate::Object<super::WorkflowLayout>,
    > = crate::Field::new("loopedTransitionContainerLayout");

    pub const START_POINT_LAYOUT: crate::Field<Self, crate::Object<super::WorkflowLayout>> =
        crate::Field::new("startPointLayout");

    pub const STATUSES: crate::Field<Self, crate::List<crate::Object<super::StatusLayoutUpdate>>> =
        crate::Field::new("statuses");

    pub const STATUS_MAPPINGS: crate::Field<
        Self,
        crate::List<crate::Object<super::StatusMappingDTO>>,
    > = crate::Field::new("statusMappings");

    pub const TRANSITIONS: crate::Field<
        Self,
        crate::List<crate::Object<super::TransitionUpdateDTO>>,
    > = crate::Field::new("transitions");

    pub const VERSION: crate::Field<Self, crate::Object<super::DocumentVersion>> =
        crate::Field::new("version");
}
