#[derive(Clone, Copy, Debug)]
pub struct BulkChangeOwnerDetails;

impl BulkChangeOwnerDetails {
    pub const AUTOFIX_NAME: crate::Field<Self, crate::Boolean> = crate::Field::new("autofixName");
    pub const OWNER_NEW: crate::Field<Self, crate::Text> = crate::Field::new("newOwner");
}
