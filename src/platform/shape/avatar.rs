#[derive(Clone, Copy, Debug)]
pub struct Avatar;

impl Avatar {
    pub const FILE_NAME: crate::Field<Self, crate::Text> = crate::Field::new("fileName");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const IS_DELETABLE: crate::Field<Self, crate::Boolean> = crate::Field::new("isDeletable");
    pub const IS_SELECTED: crate::Field<Self, crate::Boolean> = crate::Field::new("isSelected");

    pub const IS_SYSTEM_AVATAR: crate::Field<Self, crate::Boolean> =
        crate::Field::new("isSystemAvatar");

    pub const MEDIA_CLIENT_CONFIG: crate::Field<Self, crate::Any> =
        crate::Field::new("mediaClientConfig");

    pub const OWNER: crate::Field<Self, crate::Text> = crate::Field::new("owner");
    pub const URLS: crate::Field<Self, crate::Map<crate::Text>> = crate::Field::new("urls");
}
