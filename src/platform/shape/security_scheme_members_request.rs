#[derive(Clone, Copy, Debug)]
pub struct SecuritySchemeMembersRequest;

impl SecuritySchemeMembersRequest {
    pub const MEMBERS: crate::Field<
        Self,
        crate::List<crate::Object<super::SecuritySchemeLevelMemberBean>>,
    > = crate::Field::new("members");
}
