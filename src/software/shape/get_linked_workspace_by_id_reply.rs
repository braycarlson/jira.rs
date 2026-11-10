#[derive(Clone, Copy, Debug)]
pub struct GetLinkedWorkspaceByIDReply;

impl GetLinkedWorkspaceByIDReply {
    pub const UPDATED_AT: crate::Field<Self, crate::Text> = crate::Field::new("updatedAt");
    pub const WORKSPACE_ID: crate::Field<Self, crate::Text> = crate::Field::new("workspaceId");
}
