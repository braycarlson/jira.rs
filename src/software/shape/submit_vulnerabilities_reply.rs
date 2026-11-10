#[derive(Clone, Copy, Debug)]
pub struct SubmitVulnerabilitiesReply;

impl SubmitVulnerabilitiesReply {
    pub const ACCEPTED_VULNERABILITIES: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("acceptedVulnerabilities");

    pub const FAILED_VULNERABILITIES: crate::Field<
        Self,
        crate::Map<
            crate::List<crate::Object<super::SubmitVulnerabilitiesReplyFailedVulnerabilities>>,
        >,
    > = crate::Field::new("failedVulnerabilities");

    pub const UNKNOWN_ASSOCIATIONS: crate::Field<Self, crate::List<crate::Any>> =
        crate::Field::new("unknownAssociations");
}
