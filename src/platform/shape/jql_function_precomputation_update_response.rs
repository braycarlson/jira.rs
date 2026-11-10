#[derive(Clone, Copy, Debug)]
pub struct JQLFunctionPrecomputationUpdateResponse;

impl JQLFunctionPrecomputationUpdateResponse {
    pub const NOT_FOUND_PRECOMPUTATION_IDS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("notFoundPrecomputationIDs");
}
