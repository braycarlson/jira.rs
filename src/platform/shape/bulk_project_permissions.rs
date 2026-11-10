#[derive(Clone, Copy, Debug)]
pub struct BulkProjectPermissions;

impl BulkProjectPermissions {
    pub const ISSUES: crate::Field<Self, crate::List<crate::Integer>> = crate::Field::new("issues");

    pub const PERMISSIONS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("permissions");

    pub const PROJECTS: crate::Field<Self, crate::List<crate::Integer>> =
        crate::Field::new("projects");
}
