#[derive(Clone, Copy, Debug)]
pub struct BulkOperationErrorResult;

impl BulkOperationErrorResult {
    pub const ELEMENT_ERRORS: crate::Field<Self, crate::Object<super::ErrorCollection>> =
        crate::Field::new("elementErrors");

    pub const FAILED_ELEMENT_NUMBER: crate::Field<Self, crate::Integer> =
        crate::Field::new("failedElementNumber");

    pub const STATUS: crate::Field<Self, crate::Integer> = crate::Field::new("status");
}
