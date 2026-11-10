#[derive(Clone, Copy, Debug)]
pub struct BulkRedactionResponse;

impl BulkRedactionResponse {
    pub const RESULTS: crate::Field<
        Self,
        crate::List<crate::Object<super::SingleRedactionResponse>>,
    > = crate::Field::new("results");
}
