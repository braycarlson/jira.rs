#[derive(Clone, Copy, Debug)]
pub struct GetRepositoryReplyPullRequestsReviewers;

impl GetRepositoryReplyPullRequestsReviewers {
    pub const ACCOUNT_ID: crate::Field<Self, crate::Text> = crate::Field::new("accountId");

    pub const APPROVAL_STATUS: crate::Field<Self, crate::Text> =
        crate::Field::new("approvalStatus");

    pub const AVATAR: crate::Field<Self, crate::Text> = crate::Field::new("avatar");
    pub const EMAIL: crate::Field<Self, crate::Text> = crate::Field::new("email");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
    pub const URL: crate::Field<Self, crate::Text> = crate::Field::new("url");
}
