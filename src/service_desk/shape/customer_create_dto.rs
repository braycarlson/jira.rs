#[derive(Clone, Copy, Debug)]
pub struct CustomerCreateDTO;

impl CustomerCreateDTO {
    pub const DISPLAY_NAME: crate::Field<Self, crate::Text> = crate::Field::new("displayName");
    pub const EMAIL: crate::Field<Self, crate::Text> = crate::Field::new("email");
    pub const FULL_NAME: crate::Field<Self, crate::Text> = crate::Field::new("fullName");
}
