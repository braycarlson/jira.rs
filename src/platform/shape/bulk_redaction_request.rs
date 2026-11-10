#[derive(Clone, Copy, Debug)]
pub struct BulkRedactionRequest;

impl BulkRedactionRequest {
    pub const REDACTIONS: crate::Field<
        Self,
        crate::List<crate::Object<super::SingleRedactionRequest>>,
    > = crate::Field::new("redactions");
}
