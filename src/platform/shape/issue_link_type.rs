#[derive(Clone, Copy, Debug)]
pub struct IssueLinkType;

impl IssueLinkType {
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const INWARD: crate::Field<Self, crate::Text> = crate::Field::new("inward");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
    pub const OUTWARD: crate::Field<Self, crate::Text> = crate::Field::new("outward");
    pub const SELF: crate::Field<Self, crate::Text> = crate::Field::new("self");
}
