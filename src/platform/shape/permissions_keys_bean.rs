#[derive(Clone, Copy, Debug)]
pub struct PermissionsKeysBean;

impl PermissionsKeysBean {
    pub const PERMISSIONS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("permissions");
}
