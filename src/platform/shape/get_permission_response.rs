#[derive(Clone, Copy, Debug)]
pub struct GetPermissionResponse;

impl GetPermissionResponse {
    pub const HOLDER: crate::Field<Self, crate::Object<super::GetPermissionHolderResponse>> =
        crate::Field::new("holder");

    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
}
