#[derive(Clone, Copy, Debug)]
pub struct UpdateProjectsInSchemeRequestBean;

impl UpdateProjectsInSchemeRequestBean {
    pub const ADD: crate::Field<Self, crate::Object<super::PrioritySchemeChangesWithoutMappings>> =
        crate::Field::new("add");

    pub const REMOVE: crate::Field<
        Self,
        crate::Object<super::PrioritySchemeChangesWithoutMappings>,
    > = crate::Field::new("remove");
}
