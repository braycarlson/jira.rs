#[derive(Clone, Copy, Debug)]
pub struct UpdateProjectDetails;

impl UpdateProjectDetails {
    pub const ASSIGNEE_TYPE: crate::Field<Self, crate::Text> = crate::Field::new("assigneeType");
    pub const AVATAR_ID: crate::Field<Self, crate::Integer> = crate::Field::new("avatarId");
    pub const CATEGORY_ID: crate::Field<Self, crate::Integer> = crate::Field::new("categoryId");
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");

    pub const ISSUE_SECURITY_SCHEME: crate::Field<Self, crate::Integer> =
        crate::Field::new("issueSecurityScheme");

    pub const KEY: crate::Field<Self, crate::Text> = crate::Field::new("key");
    pub const LEAD: crate::Field<Self, crate::Text> = crate::Field::new("lead");
    pub const LEAD_ACCOUNT_ID: crate::Field<Self, crate::Text> = crate::Field::new("leadAccountId");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");

    pub const NOTIFICATION_SCHEME: crate::Field<Self, crate::Integer> =
        crate::Field::new("notificationScheme");

    pub const PERMISSION_SCHEME: crate::Field<Self, crate::Integer> =
        crate::Field::new("permissionScheme");

    pub const RELEASED_PROJECT_KEYS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("releasedProjectKeys");

    pub const URL: crate::Field<Self, crate::Text> = crate::Field::new("url");
}
