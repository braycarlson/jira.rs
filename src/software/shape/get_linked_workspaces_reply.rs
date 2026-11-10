#[derive(Clone, Copy, Debug)]
pub struct GetLinkedWorkspacesReply;

impl GetLinkedWorkspacesReply {
    pub const WORKSPACE_IDS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("workspaceIds");
}
