#[derive(Clone, Copy, Debug)]
pub struct BulkEditActionError;

impl BulkEditActionError {
    pub const ERRORS: crate::Field<Self, crate::Map<crate::Text>> = crate::Field::new("errors");

    pub const ERROR_MESSAGES: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("errorMessages");
}
