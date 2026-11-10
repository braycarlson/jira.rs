#[derive(Clone, Copy, Debug)]
pub struct StatusMetadata;

impl StatusMetadata {
    pub const CATEGORY: crate::Field<Self, crate::Text> = crate::Field::new("category");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
}
