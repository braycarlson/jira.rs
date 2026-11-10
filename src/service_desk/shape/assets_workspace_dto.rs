#[derive(Clone, Copy, Debug)]
pub struct AssetsWorkspaceDTO;

impl AssetsWorkspaceDTO {
    pub const WORKSPACE_ID: crate::Field<Self, crate::Text> = crate::Field::new("workspaceId");
}
