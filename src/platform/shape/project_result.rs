#[derive(Clone, Copy, Debug)]
pub struct ProjectResult;

impl ProjectResult {
    pub const ERRORS: crate::Field<
        Self,
        crate::List<crate::Object<super::PolicyApplicationViolation>>,
    > = crate::Field::new("errors");

    pub const MESSAGE: crate::Field<Self, crate::Text> = crate::Field::new("message");
    pub const PROJECT_ID: crate::Field<Self, crate::Integer> = crate::Field::new("projectId");
}
