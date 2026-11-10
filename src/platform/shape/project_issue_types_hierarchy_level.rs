#[derive(Clone, Copy, Debug)]
pub struct ProjectIssueTypesHierarchyLevel;

impl ProjectIssueTypesHierarchyLevel {
    pub const ENTITY_ID: crate::Field<Self, crate::Text> = crate::Field::new("entityId");

    pub const ISSUE_TYPES: crate::Field<Self, crate::List<crate::Object<super::IssueTypeInfo>>> =
        crate::Field::new("issueTypes");

    pub const LEVEL: crate::Field<Self, crate::Integer> = crate::Field::new("level");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
}
