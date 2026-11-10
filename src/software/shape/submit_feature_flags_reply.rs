#[derive(Clone, Copy, Debug)]
pub struct SubmitFeatureFlagsReply;

impl SubmitFeatureFlagsReply {
    pub const ACCEPTED_FEATURE_FLAGS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("acceptedFeatureFlags");

    pub const FAILED_FEATURE_FLAGS: crate::Field<
        Self,
        crate::Map<crate::List<crate::Object<super::SubmitFeatureFlagsReplyFailedFeatureFlags>>>,
    > = crate::Field::new("failedFeatureFlags");

    pub const UNKNOWN_ASSOCIATIONS: crate::Field<
        Self,
        crate::List<crate::Object<super::IssueIDOrKeysAssociation>>,
    > = crate::Field::new("unknownAssociations");

    pub const UNKNOWN_ISSUE_KEYS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("unknownIssueKeys");
}
