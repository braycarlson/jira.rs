#[derive(Clone, Copy, Debug)]
pub struct BulkOperationErrorResponse;

impl BulkOperationErrorResponse {
    pub const ERRORS: crate::Field<Self, crate::List<crate::Object<super::ErrorMessage>>> =
        crate::Field::new("errors");
}
