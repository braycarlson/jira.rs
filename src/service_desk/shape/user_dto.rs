#[derive(Clone, Copy, Debug)]
pub struct UserDTO;

impl UserDTO {
    pub const ACCOUNT_ID: crate::Field<Self, crate::Text> = crate::Field::new("accountId");
    pub const ACTIVE: crate::Field<Self, crate::Boolean> = crate::Field::new("active");
    pub const DISPLAY_NAME: crate::Field<Self, crate::Text> = crate::Field::new("displayName");
    pub const EMAIL_ADDRESS: crate::Field<Self, crate::Text> = crate::Field::new("emailAddress");
    pub const KEY: crate::Field<Self, crate::Text> = crate::Field::new("key");

    pub const LINKS: crate::Field<Self, crate::Object<super::UserLinkDTO>> =
        crate::Field::new("_links");

    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
    pub const TIME_ZONE: crate::Field<Self, crate::Text> = crate::Field::new("timeZone");
}
