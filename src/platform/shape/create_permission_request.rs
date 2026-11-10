#[derive(Clone, Copy, Debug)]
pub struct CreatePermissionRequest;

impl CreatePermissionRequest {
    pub const HOLDER: crate::Field<Self, crate::Object<super::CreatePermissionHolderRequest>> =
        crate::Field::new("holder");

    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
}
