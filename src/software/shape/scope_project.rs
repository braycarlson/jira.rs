#[derive(Clone, Copy, Debug)]
pub struct ScopeProject;

impl ScopeProject {
    pub const AVATAR_URLS: crate::Field<Self, crate::Object<super::ScopeProjectAvatarURLs>> =
        crate::Field::new("avatarUrls");

    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const KEY: crate::Field<Self, crate::Text> = crate::Field::new("key");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");

    pub const PROJECT_CATEGORY: crate::Field<
        Self,
        crate::Object<super::ScopeProjectProjectCategory>,
    > = crate::Field::new("projectCategory");

    pub const PROJECT_TYPE_KEY: crate::Field<Self, crate::Text> =
        crate::Field::new("projectTypeKey");

    pub const SELF: crate::Field<Self, crate::Text> = crate::Field::new("self");
    pub const SIMPLIFIED: crate::Field<Self, crate::Boolean> = crate::Field::new("simplified");
}
