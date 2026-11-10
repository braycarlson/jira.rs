#[derive(Clone, Copy, Debug)]
pub struct FieldSchemeToProjectsPartialFailure;

impl FieldSchemeToProjectsPartialFailure {
    pub const ERROR: crate::Field<Self, crate::Text> = crate::Field::new("error");
    pub const PROJECT_ID: crate::Field<Self, crate::Integer> = crate::Field::new("projectId");
    pub const SCHEME_ID: crate::Field<Self, crate::Integer> = crate::Field::new("schemeId");
    pub const SUCCESS: crate::Field<Self, crate::Boolean> = crate::Field::new("success");
}
