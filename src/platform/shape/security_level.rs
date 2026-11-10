#[derive(Clone, Copy, Debug)]
pub struct SecurityLevel;

impl SecurityLevel {
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");

    pub const ISSUE_SECURITY_SCHEME_ID: crate::Field<Self, crate::Text> =
        crate::Field::new("issueSecuritySchemeId");

    pub const IS_DEFAULT: crate::Field<Self, crate::Boolean> = crate::Field::new("isDefault");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
    pub const SELF: crate::Field<Self, crate::Text> = crate::Field::new("self");
}
