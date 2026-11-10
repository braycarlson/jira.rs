#[derive(Clone, Copy, Debug)]
pub struct SimplifiedIssueTransition;

impl SimplifiedIssueTransition {
    pub const TO_VALUE: crate::Field<Self, crate::Object<super::IssueTransitionStatus>> =
        crate::Field::new("to");

    pub const TRANSITION_ID: crate::Field<Self, crate::Integer> = crate::Field::new("transitionId");

    pub const TRANSITION_NAME: crate::Field<Self, crate::Text> =
        crate::Field::new("transitionName");
}
