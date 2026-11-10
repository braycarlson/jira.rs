#[derive(Clone, Copy, Debug)]
pub struct UpdatePrioritySchemeResponseBean;

impl UpdatePrioritySchemeResponseBean {
    pub const PRIORITY_SCHEME: crate::Field<
        Self,
        crate::Object<super::PrioritySchemeWithPaginatedPrioritiesAndProjects>,
    > = crate::Field::new("priorityScheme");

    pub const TASK: crate::Field<Self, crate::Object<super::TaskProgressBeanJSONNode>> =
        crate::Field::new("task");

    pub const UPDATED: crate::Field<Self, crate::Any> = crate::Field::new("updated");
}
