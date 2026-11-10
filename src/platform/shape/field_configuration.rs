#[derive(Clone, Copy, Debug)]
pub struct FieldConfiguration;

impl FieldConfiguration {
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const ID: crate::Field<Self, crate::Integer> = crate::Field::new("id");
    pub const IS_DEFAULT: crate::Field<Self, crate::Boolean> = crate::Field::new("isDefault");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
}
