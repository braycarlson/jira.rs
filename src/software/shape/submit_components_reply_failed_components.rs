#[derive(Clone, Copy, Debug)]
pub struct SubmitComponentsReplyFailedComponents;

impl SubmitComponentsReplyFailedComponents {
    pub const ERROR_TRACE_ID: crate::Field<Self, crate::Text> = crate::Field::new("errorTraceId");
    pub const MESSAGE: crate::Field<Self, crate::Text> = crate::Field::new("message");
}
