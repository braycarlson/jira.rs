#[derive(Clone, Copy, Debug)]
pub struct Dashboard;

impl Dashboard {
    pub const AUTOMATIC_REFRESH_MS: crate::Field<Self, crate::Integer> =
        crate::Field::new("automaticRefreshMs");

    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");

    pub const EDIT_PERMISSIONS: crate::Field<
        Self,
        crate::List<crate::Object<super::SharePermission>>,
    > = crate::Field::new("editPermissions");

    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const IS_FAVOURITE: crate::Field<Self, crate::Boolean> = crate::Field::new("isFavourite");
    pub const IS_WRITABLE: crate::Field<Self, crate::Boolean> = crate::Field::new("isWritable");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");

    pub const OWNER: crate::Field<Self, crate::Object<super::UserBean>> =
        crate::Field::new("owner");

    pub const POPULARITY: crate::Field<Self, crate::Integer> = crate::Field::new("popularity");
    pub const RANK: crate::Field<Self, crate::Integer> = crate::Field::new("rank");
    pub const SELF: crate::Field<Self, crate::Text> = crate::Field::new("self");

    pub const SHARE_PERMISSIONS: crate::Field<
        Self,
        crate::List<crate::Object<super::SharePermission>>,
    > = crate::Field::new("sharePermissions");

    pub const SYSTEM_DASHBOARD: crate::Field<Self, crate::Boolean> =
        crate::Field::new("systemDashboard");

    pub const VIEW: crate::Field<Self, crate::Text> = crate::Field::new("view");
}
