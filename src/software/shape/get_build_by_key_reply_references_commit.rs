#[derive(Clone, Copy, Debug)]
pub struct GetBuildByKeyReplyReferencesCommit;

impl GetBuildByKeyReplyReferencesCommit {
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const REPOSITORY_URI: crate::Field<Self, crate::Text> = crate::Field::new("repositoryUri");
}
