#[derive(Clone, Copy, Debug)]
pub struct Field;

impl Field {
    pub const ARE_OPTIONS_SUPPORTED: crate::Field<Self, crate::Boolean> =
        crate::Field::new("areOptionsSupported");

    pub const CONTEXTS_COUNT: crate::Field<Self, crate::Integer> =
        crate::Field::new("contextsCount");

    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const IS_LOCKED: crate::Field<Self, crate::Boolean> = crate::Field::new("isLocked");

    pub const IS_OPTIONS_OVER_LIMIT_COUNT: crate::Field<Self, crate::Boolean> =
        crate::Field::new("isOptionsCountOverLimit");

    pub const IS_UNSCREENABLE: crate::Field<Self, crate::Boolean> =
        crate::Field::new("isUnscreenable");

    pub const KEY: crate::Field<Self, crate::Text> = crate::Field::new("key");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");

    pub const PLANNED_DELETION_DATE: crate::Field<Self, crate::Text> =
        crate::Field::new("plannedDeletionDate");

    pub const PROJECTS_COUNT: crate::Field<Self, crate::Integer> =
        crate::Field::new("projectsCount");

    pub const SCHEMA: crate::Field<Self, crate::Object<super::JSONTypeBean>> =
        crate::Field::new("schema");

    pub const SCREENS_COUNT: crate::Field<Self, crate::Integer> = crate::Field::new("screensCount");
    pub const SEARCHER_KEY: crate::Field<Self, crate::Text> = crate::Field::new("searcherKey");
    pub const STABLE_ID: crate::Field<Self, crate::Text> = crate::Field::new("stableId");

    pub const TRANSLATED_DESCRIPTION: crate::Field<Self, crate::Text> =
        crate::Field::new("translatedDescription");

    pub const TRANSLATED_NAME: crate::Field<Self, crate::Text> =
        crate::Field::new("translatedName");

    pub const TRASHED_BY: crate::Field<Self, crate::Any> = crate::Field::new("trashedBy");
    pub const TRASHED_DATE: crate::Field<Self, crate::Text> = crate::Field::new("trashedDate");

    pub const TYPE_DISPLAY_NAME: crate::Field<Self, crate::Text> =
        crate::Field::new("typeDisplayName");

    pub const USED_LAST: crate::Field<Self, crate::Object<super::FieldLastUsed>> =
        crate::Field::new("lastUsed");
}
