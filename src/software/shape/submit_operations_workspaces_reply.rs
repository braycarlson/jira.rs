#[derive(Clone, Copy, Debug)]
pub struct SubmitOperationsWorkspacesReply;

impl SubmitOperationsWorkspacesReply {
    pub const ACCEPTED_WORKSPACE_IDS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("acceptedWorkspaceIds");
}
