#[derive(Clone, Copy, Debug)]
pub struct ContentItem;

impl ContentItem {
    pub const ENTITY_ID: crate::Field<Self, crate::Text> = crate::Field::new("entityId");
    pub const ENTITY_TYPE: crate::Field<Self, crate::Text> = crate::Field::new("entityType");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
}
