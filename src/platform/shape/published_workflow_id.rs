#[derive(Clone, Copy, Debug)]
pub struct PublishedWorkflowID;

impl PublishedWorkflowID {
    pub const ENTITY_ID: crate::Field<Self, crate::Text> = crate::Field::new("entityId");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
}
