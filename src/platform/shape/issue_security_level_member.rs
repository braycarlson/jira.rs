#[derive(Clone, Copy, Debug)]
pub struct IssueSecurityLevelMember;

impl IssueSecurityLevelMember {
    pub const HOLDER: crate::Field<Self, crate::Object<super::PermissionHolder>> =
        crate::Field::new("holder");

    pub const ID: crate::Field<Self, crate::Integer> = crate::Field::new("id");

    pub const ISSUE_SECURITY_LEVEL_ID: crate::Field<Self, crate::Integer> =
        crate::Field::new("issueSecurityLevelId");
}
