#[derive(Clone, Copy, Debug)]
pub struct BulkProjectPermissionGrants;

impl BulkProjectPermissionGrants {
    pub const ISSUES: crate::Field<Self, crate::List<crate::Integer>> = crate::Field::new("issues");
    pub const PERMISSION: crate::Field<Self, crate::Text> = crate::Field::new("permission");

    pub const PROJECTS: crate::Field<Self, crate::List<crate::Integer>> =
        crate::Field::new("projects");
}
