#[derive(Clone, Copy, Debug)]
pub struct RestrictedPermission;

impl RestrictedPermission {
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const KEY: crate::Field<Self, crate::Text> = crate::Field::new("key");
}
