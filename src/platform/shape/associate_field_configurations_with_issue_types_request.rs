#[derive(Clone, Copy, Debug)]
pub struct AssociateFieldConfigurationsWithIssueTypesRequest;

impl AssociateFieldConfigurationsWithIssueTypesRequest {
    pub const MAPPINGS: crate::Field<
        Self,
        crate::List<crate::Object<super::FieldConfigurationToIssueTypeMapping>>,
    > = crate::Field::new("mappings");
}
