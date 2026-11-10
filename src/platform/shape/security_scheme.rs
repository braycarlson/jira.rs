#[derive(Clone, Copy, Debug)]
pub struct SecurityScheme;

impl SecurityScheme {
    pub const DEFAULT_SECURITY_LEVEL_ID: crate::Field<Self, crate::Integer> =
        crate::Field::new("defaultSecurityLevelId");

    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const ID: crate::Field<Self, crate::Integer> = crate::Field::new("id");

    pub const LEVELS: crate::Field<Self, crate::List<crate::Object<super::SecurityLevel>>> =
        crate::Field::new("levels");

    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
    pub const SELF: crate::Field<Self, crate::Text> = crate::Field::new("self");
}
