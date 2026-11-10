#[derive(Clone, Copy, Debug)]
pub struct UserPickerUser;

impl UserPickerUser {
    pub const ACCOUNT_ID: crate::Field<Self, crate::Text> = crate::Field::new("accountId");
    pub const ACCOUNT_TYPE: crate::Field<Self, crate::Text> = crate::Field::new("accountType");
    pub const AVATAR_URL: crate::Field<Self, crate::Text> = crate::Field::new("avatarUrl");
    pub const DISPLAY_NAME: crate::Field<Self, crate::Text> = crate::Field::new("displayName");
    pub const HTML: crate::Field<Self, crate::Text> = crate::Field::new("html");
    pub const KEY: crate::Field<Self, crate::Text> = crate::Field::new("key");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
}
