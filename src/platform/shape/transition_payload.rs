#[derive(Clone, Copy, Debug)]
pub struct TransitionPayload;

impl TransitionPayload {
    pub const ACTIONS: crate::Field<Self, crate::List<crate::Object<super::RulePayload>>> =
        crate::Field::new("actions");

    pub const CONDITIONS: crate::Field<Self, crate::Object<super::ConditionGroupPayload>> =
        crate::Field::new("conditions");

    pub const CUSTOM_ISSUE_EVENT_ID: crate::Field<Self, crate::Text> =
        crate::Field::new("customIssueEventId");

    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");

    pub const FROM: crate::Field<Self, crate::List<crate::Object<super::FromLayoutPayload>>> =
        crate::Field::new("from");

    pub const ID: crate::Field<Self, crate::Integer> = crate::Field::new("id");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");

    pub const PROPERTIES: crate::Field<Self, crate::Map<crate::Text>> =
        crate::Field::new("properties");

    pub const TO_VALUE: crate::Field<Self, crate::Object<super::ToLayoutPayload>> =
        crate::Field::new("to");

    pub const TRANSITION_SCREEN: crate::Field<Self, crate::Object<super::RulePayload>> =
        crate::Field::new("transitionScreen");

    pub const TRIGGERS: crate::Field<Self, crate::List<crate::Object<super::RulePayload>>> =
        crate::Field::new("triggers");

    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");

    pub const VALIDATORS: crate::Field<Self, crate::List<crate::Object<super::RulePayload>>> =
        crate::Field::new("validators");
}
