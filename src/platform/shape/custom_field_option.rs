#[derive(Clone, Copy, Debug)]
pub struct CustomFieldOption;

impl CustomFieldOption {
    pub const SELF: crate::Field<Self, crate::Text> = crate::Field::new("self");
    pub const VALUE: crate::Field<Self, crate::Text> = crate::Field::new("value");
}
