#[derive(Clone, Copy, Debug)]
pub struct TransitionUpdateDTO;

impl TransitionUpdateDTO {
    pub const ACTIONS: crate::Field<
        Self,
        crate::List<crate::Object<super::WorkflowRuleConfiguration>>,
    > = crate::Field::new("actions");

    pub const CONDITIONS: crate::Field<Self, crate::Object<super::ConditionGroupUpdate>> =
        crate::Field::new("conditions");

    pub const CUSTOM_ISSUE_EVENT_ID: crate::Field<Self, crate::Text> =
        crate::Field::new("customIssueEventId");

    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");

    pub const LINKS: crate::Field<
        Self,
        crate::List<crate::Object<super::WorkflowTransitionLinks>>,
    > = crate::Field::new("links");

    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");

    pub const PROPERTIES: crate::Field<Self, crate::Map<crate::Text>> =
        crate::Field::new("properties");

    pub const TO_STATUS_REFERENCE: crate::Field<Self, crate::Text> =
        crate::Field::new("toStatusReference");

    pub const TRANSITION_SCREEN: crate::Field<
        Self,
        crate::Object<super::WorkflowRuleConfiguration>,
    > = crate::Field::new("transitionScreen");

    pub const TRIGGERS: crate::Field<Self, crate::List<crate::Object<super::WorkflowTrigger>>> =
        crate::Field::new("triggers");

    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");

    pub const VALIDATORS: crate::Field<
        Self,
        crate::List<crate::Object<super::WorkflowRuleConfiguration>>,
    > = crate::Field::new("validators");
}
