#[derive(Clone, Copy, Debug)]
pub struct GetProjectsWithFieldSchemesResponse;

impl GetProjectsWithFieldSchemesResponse {
    pub const PROJECT_ID: crate::Field<Self, crate::Integer> = crate::Field::new("projectId");
    pub const SCHEME_ID: crate::Field<Self, crate::Integer> = crate::Field::new("schemeId");
}
