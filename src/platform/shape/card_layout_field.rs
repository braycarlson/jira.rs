#[derive(Clone, Copy, Debug)]
pub struct CardLayoutField;

impl CardLayoutField {
    pub const FIELD_ID: crate::Field<Self, crate::Text> = crate::Field::new("fieldId");
    pub const ID: crate::Field<Self, crate::Integer> = crate::Field::new("id");
    pub const MODE: crate::Field<Self, crate::Text> = crate::Field::new("mode");
    pub const POSITION: crate::Field<Self, crate::Integer> = crate::Field::new("position");
}
