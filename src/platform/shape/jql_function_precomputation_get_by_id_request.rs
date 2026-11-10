#[derive(Clone, Copy, Debug)]
pub struct JQLFunctionPrecomputationGetByIDRequest;

impl JQLFunctionPrecomputationGetByIDRequest {
    pub const PRECOMPUTATION_IDS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("precomputationIDs");
}
