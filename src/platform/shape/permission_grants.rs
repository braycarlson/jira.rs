#[derive(Clone, Copy, Debug)]
pub struct PermissionGrants;

impl PermissionGrants {
    pub const EXPAND: crate::Field<Self, crate::Text> = crate::Field::new("expand");

    pub const PERMISSIONS: crate::Field<Self, crate::List<crate::Object<super::PermissionGrant>>> =
        crate::Field::new("permissions");
}
