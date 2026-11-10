#[derive(Clone, Copy, Debug)]
pub struct EntityProperty;

impl EntityProperty {
    pub const KEY: crate::Field<Self, crate::Text> = crate::Field::new("key");
    pub const SELF: crate::Field<Self, crate::Text> = crate::Field::new("self");
    pub const VALUE: crate::Field<Self, crate::Any> = crate::Field::new("value");
}
