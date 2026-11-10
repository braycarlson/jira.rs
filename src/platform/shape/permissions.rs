#[derive(Clone, Copy, Debug)]
pub struct Permissions;

impl Permissions {
    pub const PERMISSIONS: crate::Field<Self, crate::Map<crate::Object<super::UserPermission>>> =
        crate::Field::new("permissions");
}
