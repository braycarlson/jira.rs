#[derive(Clone, Copy, Debug)]
pub struct StatePolicyDocument;

impl StatePolicyDocument {
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const IS_DEFAULT: crate::Field<Self, crate::Boolean> = crate::Field::new("isDefault");

    pub const POLICY_DESCRIPTION: crate::Field<Self, crate::Text> =
        crate::Field::new("policyDescription");

    pub const POLICY_NAME: crate::Field<Self, crate::Text> = crate::Field::new("policyName");

    pub const RULES_BY_DOMAIN: crate::Field<
        Self,
        crate::Map<crate::List<crate::Object<super::StatePolicyRule>>>,
    > = crate::Field::new("rulesByDomain");

    pub const VERSION: crate::Field<Self, crate::Integer> = crate::Field::new("version");
}
