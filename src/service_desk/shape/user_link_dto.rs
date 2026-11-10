#[derive(Clone, Copy, Debug)]
pub struct UserLinkDTO;

impl UserLinkDTO {
    pub const AVATAR_URLS: crate::Field<Self, crate::Map<crate::Text>> =
        crate::Field::new("avatarUrls");

    pub const JIRA_REST: crate::Field<Self, crate::Text> = crate::Field::new("jiraRest");
    pub const SELF: crate::Field<Self, crate::Text> = crate::Field::new("self");
}
