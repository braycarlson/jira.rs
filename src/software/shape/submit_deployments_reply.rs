#[derive(Clone, Copy, Debug)]
pub struct SubmitDeploymentsReply;

impl SubmitDeploymentsReply {
    pub const ACCEPTED_DEPLOYMENTS: crate::Field<
        Self,
        crate::List<crate::Object<super::SubmitDeploymentsReplyAcceptedDeployments>>,
    > = crate::Field::new("acceptedDeployments");

    pub const REJECTED_DEPLOYMENTS: crate::Field<
        Self,
        crate::List<crate::Object<super::SubmitDeploymentsReplyRejectedDeployments>>,
    > = crate::Field::new("rejectedDeployments");

    pub const UNKNOWN_ASSOCIATIONS: crate::Field<Self, crate::List<crate::Any>> =
        crate::Field::new("unknownAssociations");

    pub const UNKNOWN_ISSUE_KEYS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("unknownIssueKeys");
}
