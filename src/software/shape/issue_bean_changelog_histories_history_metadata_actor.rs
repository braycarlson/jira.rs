#[derive(Clone, Copy, Debug)]
pub struct IssueBeanChangelogHistoriesHistoryMetadataActor;

impl IssueBeanChangelogHistoriesHistoryMetadataActor {
    pub const AVATAR_URL: crate::Field<Self, crate::Text> = crate::Field::new("avatarUrl");
    pub const DISPLAY_NAME: crate::Field<Self, crate::Text> = crate::Field::new("displayName");

    pub const DISPLAY_NAME_KEY: crate::Field<Self, crate::Text> =
        crate::Field::new("displayNameKey");

    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
    pub const URL: crate::Field<Self, crate::Text> = crate::Field::new("url");
}
