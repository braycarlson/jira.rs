#[derive(Clone, Copy, Debug)]
pub struct FieldConfigurationItem;

impl FieldConfigurationItem {
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const IS_HIDDEN: crate::Field<Self, crate::Boolean> = crate::Field::new("isHidden");
    pub const IS_REQUIRED: crate::Field<Self, crate::Boolean> = crate::Field::new("isRequired");
    pub const RENDERER: crate::Field<Self, crate::Text> = crate::Field::new("renderer");
}
