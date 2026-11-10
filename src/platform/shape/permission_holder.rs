#[derive(Clone, Copy, Debug)]
pub struct PermissionHolder;

impl PermissionHolder {
    pub const EXPAND: crate::Field<Self, crate::Text> = crate::Field::new("expand");
    pub const PARAMETER: crate::Field<Self, crate::Text> = crate::Field::new("parameter");
    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
    pub const USER: crate::Field<Self, crate::Any> = crate::Field::new("user");
    pub const VALUE: crate::Field<Self, crate::Text> = crate::Field::new("value");
}
