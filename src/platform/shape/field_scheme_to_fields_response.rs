#[derive(Clone, Copy, Debug)]
pub struct FieldSchemeToFieldsResponse;

impl FieldSchemeToFieldsResponse {
    pub const RESULTS: crate::Field<
        Self,
        crate::List<crate::Object<super::FieldSchemeToFieldsPartialFailure>>,
    > = crate::Field::new("results");
}
