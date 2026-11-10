#[derive(Clone, Copy, Debug)]
pub struct SubmitEntityReplyFailedIncidents;

impl SubmitEntityReplyFailedIncidents {
    pub const ERROR_TRACE_ID: crate::Field<Self, crate::Text> = crate::Field::new("errorTraceId");
    pub const MESSAGE: crate::Field<Self, crate::Text> = crate::Field::new("message");
}
