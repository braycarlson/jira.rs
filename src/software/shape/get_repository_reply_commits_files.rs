#[derive(Clone, Copy, Debug)]
pub struct GetRepositoryReplyCommitsFiles;

impl GetRepositoryReplyCommitsFiles {
    pub const CHANGE_TYPE: crate::Field<Self, crate::Text> = crate::Field::new("changeType");
    pub const LINES_ADDED: crate::Field<Self, crate::Integer> = crate::Field::new("linesAdded");
    pub const LINES_REMOVED: crate::Field<Self, crate::Integer> = crate::Field::new("linesRemoved");
    pub const PATH: crate::Field<Self, crate::Text> = crate::Field::new("path");
    pub const URL: crate::Field<Self, crate::Text> = crate::Field::new("url");
}
