#[derive(Clone, Copy, Debug)]
pub struct CustomFieldOptionCreate;

impl CustomFieldOptionCreate {
    pub const DISABLED: crate::Field<Self, crate::Boolean> = crate::Field::new("disabled");
    pub const OPTION_ID: crate::Field<Self, crate::Text> = crate::Field::new("optionId");
    pub const VALUE: crate::Field<Self, crate::Text> = crate::Field::new("value");
}
