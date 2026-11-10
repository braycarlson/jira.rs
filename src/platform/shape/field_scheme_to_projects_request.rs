#[derive(Clone, Copy, Debug)]
pub struct FieldSchemeToProjectsRequest;

impl FieldSchemeToProjectsRequest {
    pub const PROJECT_IDS: crate::Field<Self, crate::List<crate::Integer>> =
        crate::Field::new("projectIds");
}
