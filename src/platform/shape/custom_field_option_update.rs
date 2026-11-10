#[derive(Clone, Copy, Debug)]
pub struct CustomFieldOptionUpdate;

impl CustomFieldOptionUpdate {
    pub const DISABLED: crate::Field<Self, crate::Boolean> = crate::Field::new("disabled");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const VALUE: crate::Field<Self, crate::Text> = crate::Field::new("value");
}
