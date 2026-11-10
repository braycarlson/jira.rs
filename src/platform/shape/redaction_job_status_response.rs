#[derive(Clone, Copy, Debug)]
pub struct RedactionJobStatusResponse;

impl RedactionJobStatusResponse {
    pub const BULK_REDACTION_RESPONSE: crate::Field<
        Self,
        crate::Object<super::BulkRedactionResponse>,
    > = crate::Field::new("bulkRedactionResponse");

    pub const JOB_STATUS: crate::Field<Self, crate::Text> = crate::Field::new("jobStatus");
}
