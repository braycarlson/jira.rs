#[derive(Clone, Copy, Debug)]
pub struct AvailableWorkflowSystemRule;

impl AvailableWorkflowSystemRule {
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");

    pub const INCOMPATIBLE_RULE_KEYS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("incompatibleRuleKeys");

    pub const IS_AVAILABLE_FOR_TRANSITION_INITIAL: crate::Field<Self, crate::Boolean> =
        crate::Field::new("isAvailableForInitialTransition");

    pub const IS_VISIBLE: crate::Field<Self, crate::Boolean> = crate::Field::new("isVisible");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
    pub const RULE_KEY: crate::Field<Self, crate::Text> = crate::Field::new("ruleKey");
    pub const RULE_TYPE: crate::Field<Self, crate::Text> = crate::Field::new("ruleType");
}
