#[derive(Clone, Copy, Debug)]
pub struct ProjectIDAssociationContext;

impl ProjectIDAssociationContext {
    pub const IDENTIFIER: crate::Field<Self, crate::Any> = crate::Field::new("identifier");
    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
}
