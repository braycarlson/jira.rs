#[derive(Clone, Copy, Debug)]
pub struct ProjectRoleDetails;

impl ProjectRoleDetails {
    pub const ADMIN: crate::Field<Self, crate::Boolean> = crate::Field::new("admin");
    pub const DEFAULT: crate::Field<Self, crate::Boolean> = crate::Field::new("default");
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const ID: crate::Field<Self, crate::Integer> = crate::Field::new("id");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");

    pub const ROLE_CONFIGURABLE: crate::Field<Self, crate::Boolean> =
        crate::Field::new("roleConfigurable");

    pub const SCOPE: crate::Field<Self, crate::Object<super::Scope>> = crate::Field::new("scope");
    pub const SELF: crate::Field<Self, crate::Text> = crate::Field::new("self");

    pub const TRANSLATED_NAME: crate::Field<Self, crate::Text> =
        crate::Field::new("translatedName");

    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
}
