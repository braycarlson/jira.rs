#[derive(Clone, Copy, Debug)]
pub struct JQLFunctionPrecomputationGetByIDResponse;

impl JQLFunctionPrecomputationGetByIDResponse {
    pub const NOT_FOUND_PRECOMPUTATION_IDS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("notFoundPrecomputationIDs");

    pub const PRECOMPUTATIONS: crate::Field<
        Self,
        crate::List<crate::Object<super::JQLFunctionPrecomputationBean>>,
    > = crate::Field::new("precomputations");
}
