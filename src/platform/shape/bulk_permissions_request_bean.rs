#[derive(Clone, Copy, Debug)]
pub struct BulkPermissionsRequestBean;

impl BulkPermissionsRequestBean {
    pub const ACCOUNT_ID: crate::Field<Self, crate::Text> = crate::Field::new("accountId");

    pub const GLOBAL_PERMISSIONS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("globalPermissions");

    pub const PROJECT_PERMISSIONS: crate::Field<
        Self,
        crate::List<crate::Object<super::BulkProjectPermissions>>,
    > = crate::Field::new("projectPermissions");
}
