#[derive(Clone, Copy, Debug)]
pub struct ConnectWorkflowTransitionRule;

impl ConnectWorkflowTransitionRule {
    pub const CONFIGURATION: crate::Field<Self, crate::Object<super::RuleConfiguration>> =
        crate::Field::new("configuration");

    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const KEY: crate::Field<Self, crate::Text> = crate::Field::new("key");

    pub const TRANSITION: crate::Field<Self, crate::Object<super::WorkflowTransition>> =
        crate::Field::new("transition");
}
