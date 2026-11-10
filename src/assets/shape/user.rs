#[derive(Clone, Copy, Debug)]
pub struct User;

impl User {
    pub const AVATAR_URL: crate::Field<Self, crate::Text> = crate::Field::new("avatarUrl");
    pub const DISPLAY_NAME: crate::Field<Self, crate::Text> = crate::Field::new("displayName");
    pub const EMAIL_ADDRESS: crate::Field<Self, crate::Text> = crate::Field::new("emailAddress");
    pub const HTML: crate::Field<Self, crate::Text> = crate::Field::new("html");
    pub const IS_DELETED: crate::Field<Self, crate::Boolean> = crate::Field::new("isDeleted");
    pub const KEY: crate::Field<Self, crate::Text> = crate::Field::new("key");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
    pub const RENDERED_LINK: crate::Field<Self, crate::Text> = crate::Field::new("renderedLink");

    pub const SEEN_VERSION_LAST: crate::Field<Self, crate::Text> =
        crate::Field::new("lastSeenVersion");

    pub const SELF: crate::Field<Self, crate::Text> = crate::Field::new("self");
}
