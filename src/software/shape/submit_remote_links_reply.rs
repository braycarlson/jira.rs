#[derive(Clone, Copy, Debug)]
pub struct SubmitRemoteLinksReply;

impl SubmitRemoteLinksReply {
    pub const ACCEPTED_REMOTE_LINKS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("acceptedRemoteLinks");

    pub const REJECTED_REMOTE_LINKS: crate::Field<
        Self,
        crate::Map<crate::List<crate::Object<super::SubmitRemoteLinksReplyRejectedRemoteLinks>>>,
    > = crate::Field::new("rejectedRemoteLinks");

    pub const UNKNOWN_ASSOCIATIONS: crate::Field<Self, crate::List<crate::Any>> =
        crate::Field::new("unknownAssociations");
}
