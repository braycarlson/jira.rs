#[derive(Clone, Copy, Debug)]
pub struct PermissionGrant;

impl PermissionGrant {
    pub const HOLDER: crate::Field<Self, crate::Object<super::PermissionHolder>> =
        crate::Field::new("holder");

    pub const ID: crate::Field<Self, crate::Integer> = crate::Field::new("id");
    pub const PERMISSION: crate::Field<Self, crate::Text> = crate::Field::new("permission");
    pub const SELF: crate::Field<Self, crate::Text> = crate::Field::new("self");
}
