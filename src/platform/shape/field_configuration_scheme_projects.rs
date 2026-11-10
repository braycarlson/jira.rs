#[derive(Clone, Copy, Debug)]
pub struct FieldConfigurationSchemeProjects;

impl FieldConfigurationSchemeProjects {
    pub const FIELD_CONFIGURATION_SCHEME: crate::Field<
        Self,
        crate::Object<super::FieldConfigurationScheme>,
    > = crate::Field::new("fieldConfigurationScheme");

    pub const PROJECT_IDS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("projectIds");
}
