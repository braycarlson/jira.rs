#[derive(Clone, Copy, Debug)]
pub struct StoreDevelopmentInformationReply;

impl StoreDevelopmentInformationReply {
    pub const ACCEPTED_DEVINFO_ENTITIES: crate::Field<
        Self,
        crate::Map<crate::Object<super::StoreDevelopmentInformationReplyAcceptedDevinfoEntities>>,
    > = crate::Field::new("acceptedDevinfoEntities");

    pub const FAILED_DEVINFO_ENTITIES: crate::Field<
        Self,
        crate::Map<crate::Object<super::FailedDevelopmentEntities>>,
    > = crate::Field::new("failedDevinfoEntities");

    pub const UNKNOWN_ASSOCIATIONS: crate::Field<
        Self,
        crate::List<crate::Object<super::IssueIDOrKeysAssociation>>,
    > = crate::Field::new("unknownAssociations");

    pub const UNKNOWN_ISSUE_KEYS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("unknownIssueKeys");
}
