#[derive(Clone, Copy, Debug)]
pub struct GetRepositoryReplyCommits;

impl GetRepositoryReplyCommits {
    pub const ASSOCIATIONS: crate::Field<
        Self,
        crate::List<crate::Object<super::IssueIDOrKeysAssociation>>,
    > = crate::Field::new("associations");

    pub const AUTHOR: crate::Field<Self, crate::Object<super::GetRepositoryReplyCommitsAuthor>> =
        crate::Field::new("author");

    pub const AUTHOR_TIMESTAMP: crate::Field<Self, crate::Text> =
        crate::Field::new("authorTimestamp");

    pub const DISPLAY_ID: crate::Field<Self, crate::Text> = crate::Field::new("displayId");

    pub const FILES: crate::Field<
        Self,
        crate::List<crate::Object<super::GetRepositoryReplyCommitsFiles>>,
    > = crate::Field::new("files");

    pub const FILE_COUNT: crate::Field<Self, crate::Integer> = crate::Field::new("fileCount");
    pub const FLAGS: crate::Field<Self, crate::List<crate::Text>> = crate::Field::new("flags");
    pub const HASH: crate::Field<Self, crate::Text> = crate::Field::new("hash");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");

    pub const ISSUE_KEYS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("issueKeys");

    pub const MESSAGE: crate::Field<Self, crate::Text> = crate::Field::new("message");

    pub const UPDATE_SEQUENCE_ID: crate::Field<Self, crate::Integer> =
        crate::Field::new("updateSequenceId");

    pub const URL: crate::Field<Self, crate::Text> = crate::Field::new("url");
}
