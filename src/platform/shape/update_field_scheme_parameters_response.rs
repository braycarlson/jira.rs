#[derive(Clone, Copy, Debug)]
pub struct UpdateFieldSchemeParametersResponse;

impl UpdateFieldSchemeParametersResponse {
    pub const RESULTS: crate::Field<
        Self,
        crate::List<crate::Object<super::UpdateFieldSchemeParametersPartialFailure>>,
    > = crate::Field::new("results");
}
