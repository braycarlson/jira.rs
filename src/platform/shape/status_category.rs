#[derive(Clone, Copy, Debug)]
pub struct StatusCategory;

impl StatusCategory {
    pub const COLOR_NAME: crate::Field<Self, crate::Text> = crate::Field::new("colorName");
    pub const ID: crate::Field<Self, crate::Integer> = crate::Field::new("id");
    pub const KEY: crate::Field<Self, crate::Text> = crate::Field::new("key");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
    pub const SELF: crate::Field<Self, crate::Text> = crate::Field::new("self");
}
