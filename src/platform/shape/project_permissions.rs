#[derive(Clone, Copy, Debug)]
pub struct ProjectPermissions;

impl ProjectPermissions {
    pub const CAN_EDIT: crate::Field<Self, crate::Boolean> = crate::Field::new("canEdit");
}
