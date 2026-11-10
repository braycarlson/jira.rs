#[derive(Clone, Copy, Debug)]
pub struct GetRepositoryReply;

impl GetRepositoryReply {
    pub const AVATAR: crate::Field<Self, crate::Text> = crate::Field::new("avatar");

    pub const AVATAR_DESCRIPTION: crate::Field<Self, crate::Text> =
        crate::Field::new("avatarDescription");

    pub const BRANCHES: crate::Field<
        Self,
        crate::List<crate::Object<super::GetRepositoryReplyBranches>>,
    > = crate::Field::new("branches");

    pub const COMMITS: crate::Field<
        Self,
        crate::List<crate::Object<super::GetRepositoryReplyCommits>>,
    > = crate::Field::new("commits");

    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const FORK_OF: crate::Field<Self, crate::Text> = crate::Field::new("forkOf");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");

    pub const PULL_REQUESTS: crate::Field<
        Self,
        crate::List<crate::Object<super::GetRepositoryReplyPullRequests>>,
    > = crate::Field::new("pullRequests");

    pub const UPDATE_SEQUENCE_ID: crate::Field<Self, crate::Integer> =
        crate::Field::new("updateSequenceId");

    pub const URL: crate::Field<Self, crate::Text> = crate::Field::new("url");
}
