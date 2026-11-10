#[derive(Clone, Copy, Debug)]
pub struct UpdateFieldConfigurationSchemeDetails;

impl UpdateFieldConfigurationSchemeDetails {
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
}
