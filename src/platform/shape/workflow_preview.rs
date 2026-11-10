#[derive(Clone, Copy, Debug)]
pub struct WorkflowPreview;

impl WorkflowPreview {
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");

    pub const LOOPED_TRANSITION_CONTAINER_LAYOUT: crate::Field<
        Self,
        crate::Object<super::WorkflowPreviewLayout>,
    > = crate::Field::new("loopedTransitionContainerLayout");

    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");

    pub const QUERY_CONTEXT: crate::Field<
        Self,
        crate::List<crate::Object<super::ProjectIssueTypeQueryContext>>,
    > = crate::Field::new("queryContext");

    pub const SCOPE: crate::Field<Self, crate::Object<super::WorkflowPreviewScope>> =
        crate::Field::new("scope");

    pub const START_POINT_LAYOUT: crate::Field<Self, crate::Object<super::WorkflowPreviewLayout>> =
        crate::Field::new("startPointLayout");

    pub const STATUSES: crate::Field<
        Self,
        crate::List<crate::Object<super::WorkflowPreviewStatus>>,
    > = crate::Field::new("statuses");

    pub const TRANSITIONS: crate::Field<
        Self,
        crate::List<crate::Object<super::TransitionPreview>>,
    > = crate::Field::new("transitions");

    pub const VERSION: crate::Field<Self, crate::Object<super::WorkflowDocumentVersionBean>> =
        crate::Field::new("version");
}
