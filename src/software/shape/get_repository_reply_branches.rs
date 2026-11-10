#[derive(Clone, Copy, Debug)]
pub struct GetRepositoryReplyBranches;

impl GetRepositoryReplyBranches {
    pub const ASSOCIATIONS: crate::Field<
        Self,
        crate::List<crate::Object<super::IssueIDOrKeysAssociation>>,
    > = crate::Field::new("associations");

    pub const COMMIT_LAST: crate::Field<
        Self,
        crate::Object<super::GetRepositoryReplyBranchesLastCommit>,
    > = crate::Field::new("lastCommit");

    pub const CREATE_PULL_REQUEST_URL: crate::Field<Self, crate::Text> =
        crate::Field::new("createPullRequestUrl");

    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");

    pub const ISSUE_KEYS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("issueKeys");

    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");

    pub const UPDATE_SEQUENCE_ID: crate::Field<Self, crate::Integer> =
        crate::Field::new("updateSequenceId");

    pub const URL: crate::Field<Self, crate::Text> = crate::Field::new("url");
}
