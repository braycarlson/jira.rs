#[derive(Clone, Copy, Debug)]
pub struct FieldConfigurationItemsDetails;

impl FieldConfigurationItemsDetails {
    pub const FIELD_CONFIGURATION_ITEMS: crate::Field<
        Self,
        crate::List<crate::Object<super::FieldConfigurationItem>>,
    > = crate::Field::new("fieldConfigurationItems");
}
