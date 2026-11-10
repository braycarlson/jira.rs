#[derive(Clone, Copy, Debug)]
pub struct Visibility;

impl Visibility {
    pub const IDENTIFIER: crate::Field<Self, crate::Text> = crate::Field::new("identifier");
    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
    pub const VALUE: crate::Field<Self, crate::Text> = crate::Field::new("value");
}
