#[derive(Clone, Copy, Debug)]
pub struct GetRepositoryReplyCommitsAuthor;

impl GetRepositoryReplyCommitsAuthor {
    pub const AVATAR: crate::Field<Self, crate::Text> = crate::Field::new("avatar");
    pub const EMAIL: crate::Field<Self, crate::Text> = crate::Field::new("email");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
    pub const URL: crate::Field<Self, crate::Text> = crate::Field::new("url");
    pub const USERNAME: crate::Field<Self, crate::Text> = crate::Field::new("username");
}
