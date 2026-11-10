#[derive(Clone, Copy, Debug)]
pub struct UserBean;

impl UserBean {
    pub const ACCOUNT_ID: crate::Field<Self, crate::Text> = crate::Field::new("accountId");
    pub const ACTIVE: crate::Field<Self, crate::Boolean> = crate::Field::new("active");

    pub const AVATAR_URLS: crate::Field<Self, crate::Object<super::UserBeanAvatarURLs>> =
        crate::Field::new("avatarUrls");

    pub const DISPLAY_NAME: crate::Field<Self, crate::Text> = crate::Field::new("displayName");
    pub const KEY: crate::Field<Self, crate::Text> = crate::Field::new("key");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
    pub const SELF: crate::Field<Self, crate::Text> = crate::Field::new("self");
}
