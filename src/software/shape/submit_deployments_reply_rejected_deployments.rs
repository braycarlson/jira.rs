#[derive(Clone, Copy, Debug)]
pub struct SubmitDeploymentsReplyRejectedDeployments;

impl SubmitDeploymentsReplyRejectedDeployments {
    pub const ERRORS: crate::Field<
        Self,
        crate::List<crate::Object<super::SubmitDeploymentsReplyRejectedDeploymentsErrors>>,
    > = crate::Field::new("errors");

    pub const KEY: crate::Field<
        Self,
        crate::Object<super::SubmitDeploymentsReplyRejectedDeploymentsKey>,
    > = crate::Field::new("key");
}
