#[derive(Clone, Copy, Debug)]
pub struct CustomTemplatesProjectDetails;

impl CustomTemplatesProjectDetails {
    pub const ACCESS_LEVEL: crate::Field<Self, crate::Text> = crate::Field::new("accessLevel");

    pub const ADDITIONAL_PROPERTIES: crate::Field<Self, crate::Map<crate::Text>> =
        crate::Field::new("additionalProperties");

    pub const ASSIGNEE_TYPE: crate::Field<Self, crate::Text> = crate::Field::new("assigneeType");
    pub const AVATAR_ID: crate::Field<Self, crate::Integer> = crate::Field::new("avatarId");
    pub const CATEGORY_ID: crate::Field<Self, crate::Integer> = crate::Field::new("categoryId");
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");

    pub const ENABLE_COMPONENTS: crate::Field<Self, crate::Boolean> =
        crate::Field::new("enableComponents");

    pub const KEY: crate::Field<Self, crate::Text> = crate::Field::new("key");
    pub const LANGUAGE: crate::Field<Self, crate::Text> = crate::Field::new("language");
    pub const LEAD_ACCOUNT_ID: crate::Field<Self, crate::Text> = crate::Field::new("leadAccountId");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
    pub const URL: crate::Field<Self, crate::Text> = crate::Field::new("url");
}
