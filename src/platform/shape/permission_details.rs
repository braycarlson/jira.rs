#[derive(Clone, Copy, Debug)]
pub struct PermissionDetails;

impl PermissionDetails {
    pub const EDIT_PERMISSIONS: crate::Field<
        Self,
        crate::List<crate::Object<super::SharePermission>>,
    > = crate::Field::new("editPermissions");

    pub const SHARE_PERMISSIONS: crate::Field<
        Self,
        crate::List<crate::Object<super::SharePermission>>,
    > = crate::Field::new("sharePermissions");
}
