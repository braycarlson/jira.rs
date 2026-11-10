#[derive(Clone, Copy, Debug)]
pub struct UserPermission;

impl UserPermission {
    pub const DEPRECATED_KEY: crate::Field<Self, crate::Boolean> =
        crate::Field::new("deprecatedKey");

    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");

    pub const HAVE_PERMISSION: crate::Field<Self, crate::Boolean> =
        crate::Field::new("havePermission");

    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const KEY: crate::Field<Self, crate::Text> = crate::Field::new("key");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
}
