#[derive(Clone, Copy, Debug)]
pub struct RequestTypePermissionCheckResponse;

impl RequestTypePermissionCheckResponse {
    pub const CAN_ADMINISTER: crate::Field<Self, crate::List<crate::Integer>> =
        crate::Field::new("canAdminister");

    pub const CAN_CREATE_REQUEST: crate::Field<Self, crate::List<crate::Integer>> =
        crate::Field::new("canCreateRequest");
}
