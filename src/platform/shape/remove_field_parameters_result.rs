#[derive(Clone, Copy, Debug)]
pub struct RemoveFieldParametersResult;

impl RemoveFieldParametersResult {
    pub const RESULTS: crate::Field<
        Self,
        crate::List<crate::Object<super::SuccessOrErrorResults>>,
    > = crate::Field::new("results");
}
