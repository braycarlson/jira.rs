#[derive(Clone, Copy, Debug)]
pub struct PermissionSchemes;

impl PermissionSchemes {
    pub const PERMISSION_SCHEMES: crate::Field<
        Self,
        crate::List<crate::Object<super::PermissionScheme>>,
    > = crate::Field::new("permissionSchemes");
}
