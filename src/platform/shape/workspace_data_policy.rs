#[derive(Clone, Copy, Debug)]
pub struct WorkspaceDataPolicy;

impl WorkspaceDataPolicy {
    pub const ANY_CONTENT_BLOCKED: crate::Field<Self, crate::Boolean> =
        crate::Field::new("anyContentBlocked");
}
