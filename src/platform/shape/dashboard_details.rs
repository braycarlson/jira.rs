#[derive(Clone, Copy, Debug)]
pub struct DashboardDetails;

impl DashboardDetails {
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");

    pub const EDIT_PERMISSIONS: crate::Field<
        Self,
        crate::List<crate::Object<super::SharePermission>>,
    > = crate::Field::new("editPermissions");

    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");

    pub const SHARE_PERMISSIONS: crate::Field<
        Self,
        crate::List<crate::Object<super::SharePermission>>,
    > = crate::Field::new("sharePermissions");
}
