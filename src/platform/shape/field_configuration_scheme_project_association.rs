#[derive(Clone, Copy, Debug)]
pub struct FieldConfigurationSchemeProjectAssociation;

impl FieldConfigurationSchemeProjectAssociation {
    pub const FIELD_CONFIGURATION_SCHEME_ID: crate::Field<Self, crate::Text> =
        crate::Field::new("fieldConfigurationSchemeId");

    pub const PROJECT_ID: crate::Field<Self, crate::Text> = crate::Field::new("projectId");
}
