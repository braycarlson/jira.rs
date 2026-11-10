#[derive(Clone, Copy, Debug)]
pub struct JQLFunctionPrecomputationUpdateErrorResponse;

impl JQLFunctionPrecomputationUpdateErrorResponse {
    pub const ERROR_MESSAGES: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("errorMessages");

    pub const NOT_FOUND_PRECOMPUTATION_IDS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("notFoundPrecomputationIDs");
}
