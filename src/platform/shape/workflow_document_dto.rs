#[derive(Clone, Copy, Debug)]
pub struct WorkflowDocumentDTO;

impl WorkflowDocumentDTO {
    pub const CREATED: crate::Field<Self, crate::Text> = crate::Field::new("created");
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");

    pub const LOOPED_TRANSITION_CONTAINER_LAYOUT: crate::Field<
        Self,
        crate::Object<super::WorkflowLayout>,
    > = crate::Field::new("loopedTransitionContainerLayout");

    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");

    pub const SCOPE: crate::Field<Self, crate::Object<super::WorkflowScope>> =
        crate::Field::new("scope");

    pub const START_POINT_LAYOUT: crate::Field<Self, crate::Object<super::WorkflowLayout>> =
        crate::Field::new("startPointLayout");

    pub const STATUSES: crate::Field<
        Self,
        crate::List<crate::Object<super::WorkflowReferenceStatus>>,
    > = crate::Field::new("statuses");

    pub const TRANSITIONS: crate::Field<
        Self,
        crate::List<crate::Object<super::WorkflowTransitions>>,
    > = crate::Field::new("transitions");

    pub const UPDATED: crate::Field<Self, crate::Text> = crate::Field::new("updated");

    pub const UPDATE_AUTHOR_AAID_LAST: crate::Field<Self, crate::Text> =
        crate::Field::new("lastUpdateAuthorAAID");

    pub const VERSION: crate::Field<Self, crate::Object<super::DocumentVersion>> =
        crate::Field::new("version");
}
