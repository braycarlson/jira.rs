#[derive(Clone, Copy, Debug)]
pub struct SecurityLevelMember;

impl SecurityLevelMember {
    pub const HOLDER: crate::Field<Self, crate::Object<super::PermissionHolder>> =
        crate::Field::new("holder");

    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");

    pub const ISSUE_SECURITY_LEVEL_ID: crate::Field<Self, crate::Text> =
        crate::Field::new("issueSecurityLevelId");

    pub const ISSUE_SECURITY_SCHEME_ID: crate::Field<Self, crate::Text> =
        crate::Field::new("issueSecuritySchemeId");

    pub const MANAGED: crate::Field<Self, crate::Boolean> = crate::Field::new("managed");
}
