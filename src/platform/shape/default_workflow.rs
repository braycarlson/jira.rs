#[derive(Clone, Copy, Debug)]
pub struct DefaultWorkflow;

impl DefaultWorkflow {
    pub const UPDATE_DRAFT_IF_NEEDED: crate::Field<Self, crate::Boolean> =
        crate::Field::new("updateDraftIfNeeded");

    pub const WORKFLOW: crate::Field<Self, crate::Text> = crate::Field::new("workflow");
}
