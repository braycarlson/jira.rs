#[derive(Clone, Copy, Debug)]
pub struct ComponentWithIssueCount;

impl ComponentWithIssueCount {
    pub const ASSIGNEE: crate::Field<Self, crate::Object<super::User>> =
        crate::Field::new("assignee");

    pub const ASSIGNEE_TYPE: crate::Field<Self, crate::Text> = crate::Field::new("assigneeType");
    pub const COMPONENT_BEAN: crate::Field<Self, crate::Any> = crate::Field::new("componentBean");
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const ISSUE_COUNT: crate::Field<Self, crate::Integer> = crate::Field::new("issueCount");

    pub const IS_ASSIGNEE_TYPE_VALID: crate::Field<Self, crate::Boolean> =
        crate::Field::new("isAssigneeTypeValid");

    pub const LEAD: crate::Field<Self, crate::Object<super::User>> = crate::Field::new("lead");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
    pub const PROJECT: crate::Field<Self, crate::Text> = crate::Field::new("project");
    pub const PROJECT_ID: crate::Field<Self, crate::Integer> = crate::Field::new("projectId");

    pub const REAL_ASSIGNEE: crate::Field<Self, crate::Object<super::User>> =
        crate::Field::new("realAssignee");

    pub const REAL_ASSIGNEE_TYPE: crate::Field<Self, crate::Text> =
        crate::Field::new("realAssigneeType");

    pub const SELF: crate::Field<Self, crate::Text> = crate::Field::new("self");
}
