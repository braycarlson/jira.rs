#[derive(Clone, Copy, Debug)]
pub struct EntityPropertyDetails;

impl EntityPropertyDetails {
    pub const ENTITY_ID: crate::Field<Self, crate::Number> = crate::Field::new("entityId");
    pub const KEY: crate::Field<Self, crate::Text> = crate::Field::new("key");
    pub const VALUE: crate::Field<Self, crate::Text> = crate::Field::new("value");
}
