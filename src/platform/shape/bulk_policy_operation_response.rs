#[derive(Clone, Copy, Debug)]
pub struct BulkPolicyOperationResponse;

impl BulkPolicyOperationResponse {
    pub const FAILED: crate::Field<Self, crate::List<crate::Object<super::ProjectResult>>> =
        crate::Field::new("failed");

    pub const SUCCESSFUL: crate::Field<Self, crate::List<crate::Object<super::ProjectResult>>> =
        crate::Field::new("successful");
}
