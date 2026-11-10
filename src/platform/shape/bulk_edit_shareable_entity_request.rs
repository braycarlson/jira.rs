#[derive(Clone, Copy, Debug)]
pub struct BulkEditShareableEntityRequest;

impl BulkEditShareableEntityRequest {
    pub const ACTION: crate::Field<Self, crate::Text> = crate::Field::new("action");

    pub const CHANGE_OWNER_DETAILS: crate::Field<
        Self,
        crate::Object<super::BulkChangeOwnerDetails>,
    > = crate::Field::new("changeOwnerDetails");

    pub const ENTITY_IDS: crate::Field<Self, crate::List<crate::Integer>> =
        crate::Field::new("entityIds");

    pub const EXTEND_ADMIN_PERMISSIONS: crate::Field<Self, crate::Boolean> =
        crate::Field::new("extendAdminPermissions");

    pub const PERMISSION_DETAILS: crate::Field<Self, crate::Object<super::PermissionDetails>> =
        crate::Field::new("permissionDetails");
}
