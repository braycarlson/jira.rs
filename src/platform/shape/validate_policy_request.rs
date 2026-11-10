#[derive(Clone, Copy, Debug)]
pub struct ValidatePolicyRequest;

impl ValidatePolicyRequest {
    pub const POLICY_IDS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("policyIds");
}
