#[derive(Clone, Copy, Debug)]
pub struct WorkflowOperations;

impl WorkflowOperations {
    pub const CAN_DELETE: crate::Field<Self, crate::Boolean> = crate::Field::new("canDelete");
    pub const CAN_EDIT: crate::Field<Self, crate::Boolean> = crate::Field::new("canEdit");
}
