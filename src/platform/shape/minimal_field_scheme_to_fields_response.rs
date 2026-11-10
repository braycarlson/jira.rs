#[derive(Clone, Copy, Debug)]
pub struct MinimalFieldSchemeToFieldsResponse;

impl MinimalFieldSchemeToFieldsResponse {
    pub const RESULTS: crate::Field<
        Self,
        crate::List<crate::Object<super::MinimalFieldSchemeToFieldsPartialFailure>>,
    > = crate::Field::new("results");
}
