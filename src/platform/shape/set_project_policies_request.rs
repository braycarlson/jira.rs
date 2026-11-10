#[derive(Clone, Copy, Debug)]
pub struct SetProjectPoliciesRequest;

impl SetProjectPoliciesRequest {
    pub const POLICY_IDS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("policyIds");
}
