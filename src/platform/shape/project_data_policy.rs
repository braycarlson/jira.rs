#[derive(Clone, Copy, Debug)]
pub struct ProjectDataPolicy;

impl ProjectDataPolicy {
    pub const ANY_CONTENT_BLOCKED: crate::Field<Self, crate::Boolean> =
        crate::Field::new("anyContentBlocked");
}
