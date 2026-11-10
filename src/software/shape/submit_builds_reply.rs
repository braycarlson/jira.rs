#[derive(Clone, Copy, Debug)]
pub struct SubmitBuildsReply;

impl SubmitBuildsReply {
    pub const ACCEPTED_BUILDS: crate::Field<
        Self,
        crate::List<crate::Object<super::SubmitBuildsReplyAcceptedBuilds>>,
    > = crate::Field::new("acceptedBuilds");

    pub const REJECTED_BUILDS: crate::Field<
        Self,
        crate::List<crate::Object<super::SubmitBuildsReplyRejectedBuilds>>,
    > = crate::Field::new("rejectedBuilds");

    pub const UNKNOWN_ASSOCIATIONS: crate::Field<
        Self,
        crate::List<crate::Object<super::IssueIDOrKeysAssociation>>,
    > = crate::Field::new("unknownAssociations");

    pub const UNKNOWN_ISSUE_KEYS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("unknownIssueKeys");
}
