#[derive(Clone, Copy, Debug)]
pub struct WorkflowCapabilityPayload;

impl WorkflowCapabilityPayload {
    pub const STATUSES: crate::Field<Self, crate::List<crate::Object<super::StatusPayload>>> =
        crate::Field::new("statuses");

    pub const WORKFLOWS: crate::Field<Self, crate::List<crate::Object<super::WorkflowPayload>>> =
        crate::Field::new("workflows");

    pub const WORKFLOW_SCHEME: crate::Field<Self, crate::Object<super::WorkflowSchemePayload>> =
        crate::Field::new("workflowScheme");
}
