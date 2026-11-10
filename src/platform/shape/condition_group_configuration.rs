#[derive(Clone, Copy, Debug)]
pub struct ConditionGroupConfiguration;

impl ConditionGroupConfiguration {
    pub const CONDITIONS: crate::Field<
        Self,
        crate::List<crate::Object<super::WorkflowRuleConfiguration>>,
    > = crate::Field::new("conditions");

    pub const CONDITION_GROUPS: crate::Field<Self, crate::List<crate::Object<Self>>> =
        crate::Field::new("conditionGroups");

    pub const OPERATION: crate::Field<Self, crate::Text> = crate::Field::new("operation");
}
