#[derive(Clone, Copy, Debug)]
pub struct StatePolicyRule;

impl StatePolicyRule {
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const DOMAIN: crate::Field<Self, crate::Text> = crate::Field::new("domain");

    pub const PROPERTIES: crate::Field<Self, crate::Map<crate::Any>> =
        crate::Field::new("properties");

    pub const RULE_ID: crate::Field<Self, crate::Text> = crate::Field::new("ruleId");
    pub const RULE_TYPE_ID: crate::Field<Self, crate::Text> = crate::Field::new("ruleTypeId");
}
