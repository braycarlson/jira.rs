#[derive(Clone, Copy, Debug)]
pub struct WorkflowSchemeUpdateRequiredMappingsResponse;

impl WorkflowSchemeUpdateRequiredMappingsResponse {
    pub const STATUSES: crate::Field<Self, crate::List<crate::Object<super::StatusMetadata>>> =
        crate::Field::new("statuses");

    pub const STATUSES_PER_WORKFLOW: crate::Field<
        Self,
        crate::List<crate::Object<super::StatusesPerWorkflow>>,
    > = crate::Field::new("statusesPerWorkflow");

    pub const STATUS_MAPPINGS_BY_ISSUE_TYPES: crate::Field<
        Self,
        crate::List<crate::Object<super::RequiredMappingByIssueType>>,
    > = crate::Field::new("statusMappingsByIssueTypes");

    pub const STATUS_MAPPINGS_BY_WORKFLOWS: crate::Field<
        Self,
        crate::List<crate::Object<super::RequiredMappingByWorkflows>>,
    > = crate::Field::new("statusMappingsByWorkflows");
}
