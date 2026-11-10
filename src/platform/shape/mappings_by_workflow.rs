#[derive(Clone, Copy, Debug)]
pub struct MappingsByWorkflow;

impl MappingsByWorkflow {
    pub const STATUS_MAPPINGS: crate::Field<
        Self,
        crate::List<crate::Object<super::WorkflowAssociationStatusMapping>>,
    > = crate::Field::new("statusMappings");

    pub const WORKFLOW_ID_NEW: crate::Field<Self, crate::Text> = crate::Field::new("newWorkflowId");
    pub const WORKFLOW_ID_OLD: crate::Field<Self, crate::Text> = crate::Field::new("oldWorkflowId");
}
