#[derive(Clone, Copy, Debug)]
pub struct GetWorkspacesReply;

impl GetWorkspacesReply {
    pub const WORKSPACE_IDS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("workspaceIds");
}
