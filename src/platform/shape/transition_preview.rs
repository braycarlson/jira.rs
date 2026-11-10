#[derive(Clone, Copy, Debug)]
pub struct TransitionPreview;

impl TransitionPreview {
    pub const ACTIONS: crate::Field<
        Self,
        crate::List<crate::Object<super::PreviewRuleConfiguration>>,
    > = crate::Field::new("actions");

    pub const CONDITIONS: crate::Field<
        Self,
        crate::Object<super::PreviewConditionGroupConfiguration>,
    > = crate::Field::new("conditions");

    pub const CUSTOM_ISSUE_EVENT_ID: crate::Field<Self, crate::Text> =
        crate::Field::new("customIssueEventId");

    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");

    pub const LINKS: crate::Field<Self, crate::List<crate::Object<super::TransitionLink>>> =
        crate::Field::new("links");

    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");

    pub const TO_STATUS_REFERENCE: crate::Field<Self, crate::Text> =
        crate::Field::new("toStatusReference");

    pub const TRANSITION_SCREEN: crate::Field<
        Self,
        crate::Object<super::PreviewRuleConfiguration>,
    > = crate::Field::new("transitionScreen");

    pub const TRIGGERS: crate::Field<Self, crate::List<crate::Object<super::PreviewTrigger>>> =
        crate::Field::new("triggers");

    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");

    pub const VALIDATORS: crate::Field<
        Self,
        crate::List<crate::Object<super::PreviewRuleConfiguration>>,
    > = crate::Field::new("validators");
}
