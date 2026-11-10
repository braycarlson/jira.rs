#[derive(Clone, Copy, Debug)]
pub struct WorkflowAssociationStatusMapping;

impl WorkflowAssociationStatusMapping {
    pub const STATUS_ID_NEW: crate::Field<Self, crate::Text> = crate::Field::new("newStatusId");
    pub const STATUS_ID_OLD: crate::Field<Self, crate::Text> = crate::Field::new("oldStatusId");
}
