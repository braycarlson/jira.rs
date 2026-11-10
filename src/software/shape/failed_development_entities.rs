#[derive(Clone, Copy, Debug)]
pub struct FailedDevelopmentEntities;

impl FailedDevelopmentEntities {
    pub const BRANCHES: crate::Field<
        Self,
        crate::List<crate::Object<super::FailedDevelopmentEntitiesBranches>>,
    > = crate::Field::new("branches");

    pub const COMMITS: crate::Field<
        Self,
        crate::List<crate::Object<super::FailedDevelopmentEntitiesCommits>>,
    > = crate::Field::new("commits");

    pub const ERROR_MESSAGES: crate::Field<
        Self,
        crate::List<crate::Object<super::FailedDevelopmentEntitiesErrorMessages>>,
    > = crate::Field::new("errorMessages");

    pub const PULL_REQUESTS: crate::Field<
        Self,
        crate::List<crate::Object<super::FailedDevelopmentEntitiesPullRequests>>,
    > = crate::Field::new("pullRequests");
}
