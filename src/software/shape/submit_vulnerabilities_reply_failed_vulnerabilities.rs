#[derive(Clone, Copy, Debug)]
pub struct SubmitVulnerabilitiesReplyFailedVulnerabilities;

impl SubmitVulnerabilitiesReplyFailedVulnerabilities {
    pub const ERROR_TRACE_ID: crate::Field<Self, crate::Text> = crate::Field::new("errorTraceId");
    pub const MESSAGE: crate::Field<Self, crate::Text> = crate::Field::new("message");
}
