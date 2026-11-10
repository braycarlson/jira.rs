#[derive(Clone, Copy, Debug)]
pub struct ProjectComponent;

impl ProjectComponent {
    pub const ARI: crate::Field<Self, crate::Text> = crate::Field::new("ari");

    pub const ASSIGNEE: crate::Field<Self, crate::Object<super::User>> =
        crate::Field::new("assignee");

    pub const ASSIGNEE_TYPE: crate::Field<Self, crate::Text> = crate::Field::new("assigneeType");
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");

    pub const IS_ASSIGNEE_TYPE_VALID: crate::Field<Self, crate::Boolean> =
        crate::Field::new("isAssigneeTypeValid");

    pub const LEAD: crate::Field<Self, crate::Object<super::User>> = crate::Field::new("lead");
    pub const LEAD_ACCOUNT_ID: crate::Field<Self, crate::Text> = crate::Field::new("leadAccountId");
    pub const LEAD_USER_NAME: crate::Field<Self, crate::Text> = crate::Field::new("leadUserName");
    pub const METADATA: crate::Field<Self, crate::Map<crate::Text>> = crate::Field::new("metadata");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
    pub const PROJECT: crate::Field<Self, crate::Text> = crate::Field::new("project");
    pub const PROJECT_ID: crate::Field<Self, crate::Integer> = crate::Field::new("projectId");

    pub const REAL_ASSIGNEE: crate::Field<Self, crate::Object<super::User>> =
        crate::Field::new("realAssignee");

    pub const REAL_ASSIGNEE_TYPE: crate::Field<Self, crate::Text> =
        crate::Field::new("realAssigneeType");

    pub const SELF: crate::Field<Self, crate::Text> = crate::Field::new("self");
}
