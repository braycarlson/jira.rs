#[derive(Clone, Copy, Debug)]
pub struct BulkPermissionGrants;

impl BulkPermissionGrants {
    pub const GLOBAL_PERMISSIONS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("globalPermissions");

    pub const PROJECT_PERMISSIONS: crate::Field<
        Self,
        crate::List<crate::Object<super::BulkProjectPermissionGrants>>,
    > = crate::Field::new("projectPermissions");
}
