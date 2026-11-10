#[derive(Clone, Copy, Debug)]
pub struct PolicyApplicationViolation;

impl PolicyApplicationViolation {
    pub const ENTITY_ID: crate::Field<Self, crate::Text> = crate::Field::new("entityId");
    pub const ENTITY_NAME: crate::Field<Self, crate::Text> = crate::Field::new("entityName");
    pub const ENTITY_TYPE: crate::Field<Self, crate::Text> = crate::Field::new("entityType");

    pub const REMEDIATION_HINT: crate::Field<Self, crate::Text> =
        crate::Field::new("remediationHint");

    pub const RULE_TYPE_ID: crate::Field<Self, crate::Text> = crate::Field::new("ruleTypeId");
}
