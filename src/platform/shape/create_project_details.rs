#[derive(Clone, Copy, Debug)]
pub struct CreateProjectDetails;

impl CreateProjectDetails {
    pub const ASSIGNEE_TYPE: crate::Field<Self, crate::Text> = crate::Field::new("assigneeType");
    pub const AVATAR_ID: crate::Field<Self, crate::Integer> = crate::Field::new("avatarId");
    pub const CATEGORY_ID: crate::Field<Self, crate::Integer> = crate::Field::new("categoryId");
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");

    pub const FIELD_CONFIGURATION_SCHEME: crate::Field<Self, crate::Integer> =
        crate::Field::new("fieldConfigurationScheme");

    pub const FIELD_SCHEME: crate::Field<Self, crate::Integer> = crate::Field::new("fieldScheme");

    pub const ISSUE_SECURITY_SCHEME: crate::Field<Self, crate::Integer> =
        crate::Field::new("issueSecurityScheme");

    pub const ISSUE_TYPE_SCHEME: crate::Field<Self, crate::Integer> =
        crate::Field::new("issueTypeScheme");

    pub const ISSUE_TYPE_SCREEN_SCHEME: crate::Field<Self, crate::Integer> =
        crate::Field::new("issueTypeScreenScheme");

    pub const KEY: crate::Field<Self, crate::Text> = crate::Field::new("key");
    pub const LEAD: crate::Field<Self, crate::Text> = crate::Field::new("lead");
    pub const LEAD_ACCOUNT_ID: crate::Field<Self, crate::Text> = crate::Field::new("leadAccountId");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");

    pub const NOTIFICATION_SCHEME: crate::Field<Self, crate::Integer> =
        crate::Field::new("notificationScheme");

    pub const PERMISSION_SCHEME: crate::Field<Self, crate::Integer> =
        crate::Field::new("permissionScheme");

    pub const PROJECT_TEMPLATE_KEY: crate::Field<Self, crate::Text> =
        crate::Field::new("projectTemplateKey");

    pub const PROJECT_TYPE_KEY: crate::Field<Self, crate::Text> =
        crate::Field::new("projectTypeKey");

    pub const URL: crate::Field<Self, crate::Text> = crate::Field::new("url");

    pub const WORKFLOW_SCHEME: crate::Field<Self, crate::Integer> =
        crate::Field::new("workflowScheme");
}
