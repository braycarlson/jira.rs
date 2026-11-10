#[derive(Clone, Copy, Debug)]
pub struct ProjectIssueCreateMetadata;

impl ProjectIssueCreateMetadata {
    pub const AVATAR_URLS: crate::Field<Self, crate::Object<super::AvatarURLsBean>> =
        crate::Field::new("avatarUrls");

    pub const EXPAND: crate::Field<Self, crate::Text> = crate::Field::new("expand");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");

    pub const ISSUETYPES: crate::Field<
        Self,
        crate::List<crate::Object<super::IssueTypeIssueCreateMetadata>>,
    > = crate::Field::new("issuetypes");

    pub const KEY: crate::Field<Self, crate::Text> = crate::Field::new("key");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
    pub const SELF: crate::Field<Self, crate::Text> = crate::Field::new("self");
}
