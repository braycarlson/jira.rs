#[derive(Clone, Copy, Debug)]
pub struct AssociationContextObject;

impl AssociationContextObject {
    pub const IDENTIFIER: crate::Field<Self, crate::Any> = crate::Field::new("identifier");
    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
}
