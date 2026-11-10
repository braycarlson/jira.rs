#[derive(Clone, Copy, Debug)]
pub struct PrioritySchemeWithPaginatedPrioritiesAndProjects;

impl PrioritySchemeWithPaginatedPrioritiesAndProjects {
    pub const DEFAULT: crate::Field<Self, crate::Boolean> = crate::Field::new("default");

    pub const DEFAULT_PRIORITY_ID: crate::Field<Self, crate::Text> =
        crate::Field::new("defaultPriorityId");

    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const IS_DEFAULT: crate::Field<Self, crate::Boolean> = crate::Field::new("isDefault");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");

    pub const PRIORITIES: crate::Field<Self, crate::Object<super::PageBeanPriorityWithSequence>> =
        crate::Field::new("priorities");

    pub const PRIORITIES_LIMIT: crate::Field<Self, crate::Integer> =
        crate::Field::new("prioritiesLimit");

    pub const PROJECTS: crate::Field<Self, crate::Object<super::PageBeanProjectDetails>> =
        crate::Field::new("projects");

    pub const SELF: crate::Field<Self, crate::Text> = crate::Field::new("self");
}
