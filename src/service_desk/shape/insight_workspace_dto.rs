#[derive(Clone, Copy, Debug)]
pub struct InsightWorkspaceDTO;

impl InsightWorkspaceDTO {
    pub const WORKSPACE_ID: crate::Field<Self, crate::Text> = crate::Field::new("workspaceId");
}
