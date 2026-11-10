#[derive(Clone, Copy, Debug)]
pub struct AvailableWorkflowTriggers;

impl AvailableWorkflowTriggers {
    pub const AVAILABLE_TYPES: crate::Field<
        Self,
        crate::List<crate::Object<super::AvailableWorkflowTriggerTypes>>,
    > = crate::Field::new("availableTypes");

    pub const RULE_KEY: crate::Field<Self, crate::Text> = crate::Field::new("ruleKey");
}
