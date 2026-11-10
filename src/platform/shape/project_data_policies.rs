#[derive(Clone, Copy, Debug)]
pub struct ProjectDataPolicies;

impl ProjectDataPolicies {
    pub const PROJECT_DATA_POLICIES: crate::Field<
        Self,
        crate::List<crate::Object<super::ProjectWithDataPolicy>>,
    > = crate::Field::new("projectDataPolicies");
}
