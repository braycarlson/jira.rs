#[derive(Clone, Copy, Debug)]
pub struct GroupLabel;

impl GroupLabel {
    pub const TEXT: crate::Field<Self, crate::Text> = crate::Field::new("text");
    pub const TITLE: crate::Field<Self, crate::Text> = crate::Field::new("title");
    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
}
