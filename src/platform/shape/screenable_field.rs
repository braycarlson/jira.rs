#[derive(Clone, Copy, Debug)]
pub struct ScreenableField;

impl ScreenableField {
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
}
