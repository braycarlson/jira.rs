#[derive(Clone, Copy, Debug)]
pub struct MappingsByIssueTypeOverride;

impl MappingsByIssueTypeOverride {
    pub const ISSUE_TYPE_ID: crate::Field<Self, crate::Text> = crate::Field::new("issueTypeId");

    pub const STATUS_MAPPINGS: crate::Field<
        Self,
        crate::List<crate::Object<super::WorkflowAssociationStatusMapping>>,
    > = crate::Field::new("statusMappings");
}
