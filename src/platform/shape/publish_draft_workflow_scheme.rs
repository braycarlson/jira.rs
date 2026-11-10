#[derive(Clone, Copy, Debug)]
pub struct PublishDraftWorkflowScheme;

impl PublishDraftWorkflowScheme {
    pub const STATUS_MAPPINGS: crate::Field<
        Self,
        crate::List<crate::Object<super::StatusMapping>>,
    > = crate::Field::new("statusMappings");
}
