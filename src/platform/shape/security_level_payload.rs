#[derive(Clone, Copy, Debug)]
pub struct SecurityLevelPayload;

impl SecurityLevelPayload {
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const IS_DEFAULT: crate::Field<Self, crate::Boolean> = crate::Field::new("isDefault");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");

    pub const PCRI: crate::Field<Self, crate::Object<super::ProjectCreateResourceIdentifier>> =
        crate::Field::new("pcri");

    pub const SECURITY_LEVEL_MEMBERS: crate::Field<
        Self,
        crate::List<crate::Object<super::SecurityLevelMemberPayload>>,
    > = crate::Field::new("securityLevelMembers");
}
