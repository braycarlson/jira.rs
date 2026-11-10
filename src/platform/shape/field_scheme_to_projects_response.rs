#[derive(Clone, Copy, Debug)]
pub struct FieldSchemeToProjectsResponse;

impl FieldSchemeToProjectsResponse {
    pub const RESULTS: crate::Field<
        Self,
        crate::List<crate::Object<super::FieldSchemeToProjectsPartialFailure>>,
    > = crate::Field::new("results");
}
