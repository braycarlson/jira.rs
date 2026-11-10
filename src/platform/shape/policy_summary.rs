#[derive(Clone, Copy, Debug)]
pub struct PolicySummary;

impl PolicySummary {
    pub const APPLIED_PROJECT_IDS: crate::Field<Self, crate::List<crate::Integer>> =
        crate::Field::new("appliedProjectIds");

    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const IS_DEFAULT: crate::Field<Self, crate::Boolean> = crate::Field::new("isDefault");

    pub const POLICY_DESCRIPTION: crate::Field<Self, crate::Text> =
        crate::Field::new("policyDescription");

    pub const POLICY_NAME: crate::Field<Self, crate::Text> = crate::Field::new("policyName");
    pub const VERSION: crate::Field<Self, crate::Integer> = crate::Field::new("version");
}
