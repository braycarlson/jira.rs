#[derive(Clone, Copy, Debug)]
pub struct User;

impl User {
    pub const ACCOUNT_ID: crate::Field<Self, crate::Text> = crate::Field::new("accountId");
    pub const ACCOUNT_TYPE: crate::Field<Self, crate::Text> = crate::Field::new("accountType");
    pub const ACTIVE: crate::Field<Self, crate::Boolean> = crate::Field::new("active");

    pub const APPLICATION_ROLES: crate::Field<
        Self,
        crate::Object<super::SimpleListWrapperApplicationRole>,
    > = crate::Field::new("applicationRoles");

    pub const APP_TYPE: crate::Field<Self, crate::Text> = crate::Field::new("appType");

    pub const AVATAR_URLS: crate::Field<Self, crate::Object<super::AvatarURLsBean>> =
        crate::Field::new("avatarUrls");

    pub const DISPLAY_NAME: crate::Field<Self, crate::Text> = crate::Field::new("displayName");
    pub const EMAIL_ADDRESS: crate::Field<Self, crate::Text> = crate::Field::new("emailAddress");
    pub const EXPAND: crate::Field<Self, crate::Text> = crate::Field::new("expand");

    pub const GROUPS: crate::Field<Self, crate::Object<super::SimpleListWrapperGroupName>> =
        crate::Field::new("groups");

    pub const GUEST: crate::Field<Self, crate::Boolean> = crate::Field::new("guest");
    pub const KEY: crate::Field<Self, crate::Text> = crate::Field::new("key");
    pub const LOCALE: crate::Field<Self, crate::Text> = crate::Field::new("locale");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
    pub const SELF: crate::Field<Self, crate::Text> = crate::Field::new("self");
    pub const TIME_ZONE: crate::Field<Self, crate::Text> = crate::Field::new("timeZone");
}
