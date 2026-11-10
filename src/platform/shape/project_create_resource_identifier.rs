#[derive(Clone, Copy, Debug)]
pub struct ProjectCreateResourceIdentifier;

impl ProjectCreateResourceIdentifier {
    pub const AN_ID: crate::Field<Self, crate::Boolean> = crate::Field::new("anID");
    pub const AREFERENCE: crate::Field<Self, crate::Boolean> = crate::Field::new("areference");
    pub const ENTITY_ID: crate::Field<Self, crate::Text> = crate::Field::new("entityId");
    pub const ENTITY_TYPE: crate::Field<Self, crate::Text> = crate::Field::new("entityType");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
}
