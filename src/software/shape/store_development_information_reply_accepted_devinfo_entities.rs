#[derive(Clone, Copy, Debug)]
pub struct StoreDevelopmentInformationReplyAcceptedDevinfoEntities;

impl StoreDevelopmentInformationReplyAcceptedDevinfoEntities {
    pub const BRANCHES: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("branches");

    pub const COMMITS: crate::Field<Self, crate::List<crate::Text>> = crate::Field::new("commits");

    pub const PULL_REQUESTS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("pullRequests");
}
