#[derive(Clone, Copy, Debug)]
pub struct PermissionGrantDTO;

impl PermissionGrantDTO {
    pub const APPLICATION_ACCESS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("applicationAccess");

    pub const GROUPS: crate::Field<
        Self,
        crate::List<crate::Object<super::ProjectCreateResourceIdentifier>>,
    > = crate::Field::new("groups");

    pub const GROUP_CUSTOM_FIELDS: crate::Field<
        Self,
        crate::List<crate::Object<super::ProjectCreateResourceIdentifier>>,
    > = crate::Field::new("groupCustomFields");

    pub const PERMISSION_KEYS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("permissionKeys");

    pub const PROJECT_ROLES: crate::Field<
        Self,
        crate::List<crate::Object<super::ProjectCreateResourceIdentifier>>,
    > = crate::Field::new("projectRoles");

    pub const SPECIAL_GRANTS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("specialGrants");

    pub const USERS: crate::Field<
        Self,
        crate::List<crate::Object<super::ProjectCreateResourceIdentifier>>,
    > = crate::Field::new("users");

    pub const USER_CUSTOM_FIELDS: crate::Field<
        Self,
        crate::List<crate::Object<super::ProjectCreateResourceIdentifier>>,
    > = crate::Field::new("userCustomFields");
}
