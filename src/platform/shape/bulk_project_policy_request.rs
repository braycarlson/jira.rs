#[derive(Clone, Copy, Debug)]
pub struct BulkProjectPolicyRequest;

impl BulkProjectPolicyRequest {
    pub const PROJECT_IDS: crate::Field<Self, crate::List<crate::Integer>> =
        crate::Field::new("projectIds");
}
