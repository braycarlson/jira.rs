#[derive(Clone, Copy, Debug)]
pub struct BulkTransitionSubmitInput;

impl BulkTransitionSubmitInput {
    pub const SELECTED_ISSUE_IDS_OR_KEYS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("selectedIssueIdsOrKeys");

    pub const TRANSITION_ID: crate::Field<Self, crate::Text> = crate::Field::new("transitionId");
}
