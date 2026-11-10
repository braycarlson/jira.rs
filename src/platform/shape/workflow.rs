#[derive(Clone, Copy, Debug)]
pub struct Workflow;

impl Workflow {
    pub const CREATED: crate::Field<Self, crate::Text> = crate::Field::new("created");
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");

    pub const HAS_DRAFT_WORKFLOW: crate::Field<Self, crate::Boolean> =
        crate::Field::new("hasDraftWorkflow");

    pub const ID: crate::Field<Self, crate::Object<super::PublishedWorkflowID>> =
        crate::Field::new("id");

    pub const IS_DEFAULT: crate::Field<Self, crate::Boolean> = crate::Field::new("isDefault");

    pub const OPERATIONS: crate::Field<Self, crate::Object<super::WorkflowOperations>> =
        crate::Field::new("operations");

    pub const PROJECTS: crate::Field<Self, crate::List<crate::Object<super::ProjectDetails>>> =
        crate::Field::new("projects");

    pub const SCHEMES: crate::Field<Self, crate::List<crate::Object<super::WorkflowSchemeIDName>>> =
        crate::Field::new("schemes");

    pub const STATUSES: crate::Field<Self, crate::List<crate::Object<super::WorkflowStatus>>> =
        crate::Field::new("statuses");

    pub const TRANSITIONS: crate::Field<Self, crate::List<crate::Object<super::Transition>>> =
        crate::Field::new("transitions");

    pub const UPDATED: crate::Field<Self, crate::Text> = crate::Field::new("updated");
}
