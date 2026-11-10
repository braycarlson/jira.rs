#[derive(Clone, Copy, Debug)]
pub struct WorkflowSchemePayload;

impl WorkflowSchemePayload {
    pub const DEFAULT_WORKFLOW: crate::Field<
        Self,
        crate::Object<super::ProjectCreateResourceIdentifier>,
    > = crate::Field::new("defaultWorkflow");

    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");

    pub const EXPLICIT_MAPPINGS: crate::Field<
        Self,
        crate::Map<crate::Object<super::ProjectCreateResourceIdentifier>>,
    > = crate::Field::new("explicitMappings");

    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
    pub const ON_CONFLICT: crate::Field<Self, crate::Text> = crate::Field::new("onConflict");

    pub const PCRI: crate::Field<Self, crate::Object<super::ProjectCreateResourceIdentifier>> =
        crate::Field::new("pcri");
}
