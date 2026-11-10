#[derive(Clone, Copy, Debug)]
pub struct RequestTypePermissionCheckRequestDTO;

impl RequestTypePermissionCheckRequestDTO {
    pub const ACCOUNT_ID: crate::Field<Self, crate::Text> = crate::Field::new("accountId");

    pub const PERMISSIONS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("permissions");

    pub const REQUEST_TYPE_IDS: crate::Field<Self, crate::List<crate::Integer>> =
        crate::Field::new("requestTypeIds");
}
