#[derive(Clone, Copy, Debug)]
pub struct GetRepositoryReplyPullRequests;

impl GetRepositoryReplyPullRequests {
    pub const ASSOCIATIONS: crate::Field<
        Self,
        crate::List<crate::Object<super::IssueIDOrKeysAssociation>>,
    > = crate::Field::new("associations");

    pub const AUTHOR: crate::Field<
        Self,
        crate::Object<super::GetRepositoryReplyPullRequestsAuthor>,
    > = crate::Field::new("author");

    pub const COMMENT_COUNT: crate::Field<Self, crate::Integer> = crate::Field::new("commentCount");

    pub const DESTINATION_BRANCH: crate::Field<Self, crate::Text> =
        crate::Field::new("destinationBranch");

    pub const DESTINATION_BRANCH_URL: crate::Field<Self, crate::Text> =
        crate::Field::new("destinationBranchUrl");

    pub const DISPLAY_ID: crate::Field<Self, crate::Text> = crate::Field::new("displayId");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");

    pub const ISSUE_KEYS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("issueKeys");

    pub const REVIEWERS: crate::Field<
        Self,
        crate::List<crate::Object<super::GetRepositoryReplyPullRequestsReviewers>>,
    > = crate::Field::new("reviewers");

    pub const SOURCE_BRANCH: crate::Field<Self, crate::Text> = crate::Field::new("sourceBranch");

    pub const SOURCE_BRANCH_URL: crate::Field<Self, crate::Text> =
        crate::Field::new("sourceBranchUrl");

    pub const STATUS: crate::Field<Self, crate::Text> = crate::Field::new("status");
    pub const TITLE: crate::Field<Self, crate::Text> = crate::Field::new("title");
    pub const UPDATE_LAST: crate::Field<Self, crate::Text> = crate::Field::new("lastUpdate");

    pub const UPDATE_SEQUENCE_ID: crate::Field<Self, crate::Integer> =
        crate::Field::new("updateSequenceId");

    pub const URL: crate::Field<Self, crate::Text> = crate::Field::new("url");
}
