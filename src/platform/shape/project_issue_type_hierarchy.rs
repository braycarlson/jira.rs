#[derive(Clone, Copy, Debug)]
pub struct ProjectIssueTypeHierarchy;

impl ProjectIssueTypeHierarchy {
    pub const HIERARCHY: crate::Field<
        Self,
        crate::List<crate::Object<super::ProjectIssueTypesHierarchyLevel>>,
    > = crate::Field::new("hierarchy");

    pub const PROJECT_ENTITY_ID: crate::Field<Self, crate::Text> =
        crate::Field::new("projectEntityId");

    pub const PROJECT_ID: crate::Field<Self, crate::Integer> = crate::Field::new("projectId");
    pub const VERSION_TOKEN: crate::Field<Self, crate::Any> = crate::Field::new("versionToken");
}
