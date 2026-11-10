#[derive(Clone, Copy, Debug)]
pub struct DeleteFieldAssociationSchemeResponse;

impl DeleteFieldAssociationSchemeResponse {
    pub const DELETED: crate::Field<Self, crate::Boolean> = crate::Field::new("deleted");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
}
