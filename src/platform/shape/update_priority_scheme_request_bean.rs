#[derive(Clone, Copy, Debug)]
pub struct UpdatePrioritySchemeRequestBean;

impl UpdatePrioritySchemeRequestBean {
    pub const DEFAULT_PRIORITY_ID: crate::Field<Self, crate::Integer> =
        crate::Field::new("defaultPriorityId");

    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");

    pub const MAPPINGS: crate::Field<Self, crate::Object<super::PriorityMapping>> =
        crate::Field::new("mappings");

    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");

    pub const PRIORITIES: crate::Field<
        Self,
        crate::Object<super::UpdatePrioritiesInSchemeRequestBean>,
    > = crate::Field::new("priorities");

    pub const PROJECTS: crate::Field<
        Self,
        crate::Object<super::UpdateProjectsInSchemeRequestBean>,
    > = crate::Field::new("projects");
}
