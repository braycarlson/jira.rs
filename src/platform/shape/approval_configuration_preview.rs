#[derive(Clone, Copy, Debug)]
pub struct ApprovalConfigurationPreview;

impl ApprovalConfigurationPreview {
    pub const ACTIVE: crate::Field<Self, crate::Text> = crate::Field::new("active");

    pub const TRANSITION_APPROVED: crate::Field<Self, crate::Text> =
        crate::Field::new("transitionApproved");

    pub const TRANSITION_REJECTED: crate::Field<Self, crate::Text> =
        crate::Field::new("transitionRejected");
}
