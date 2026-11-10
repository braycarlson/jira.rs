#[derive(Clone, Copy, Debug)]
pub struct BoardLocationBean;

impl BoardLocationBean {
    pub const AVATAR_URI: crate::Field<Self, crate::Text> = crate::Field::new("avatarURI");
    pub const DISPLAY_NAME: crate::Field<Self, crate::Text> = crate::Field::new("displayName");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
    pub const PROJECT_ID: crate::Field<Self, crate::Integer> = crate::Field::new("projectId");
    pub const PROJECT_KEY: crate::Field<Self, crate::Text> = crate::Field::new("projectKey");
    pub const PROJECT_NAME: crate::Field<Self, crate::Text> = crate::Field::new("projectName");

    pub const PROJECT_TYPE_KEY: crate::Field<Self, crate::Text> =
        crate::Field::new("projectTypeKey");

    pub const USER_ACCOUNT_ID: crate::Field<Self, crate::Text> = crate::Field::new("userAccountId");
    pub const USER_ID: crate::Field<Self, crate::Integer> = crate::Field::new("userId");
}
