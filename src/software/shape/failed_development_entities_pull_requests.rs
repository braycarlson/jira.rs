#[derive(Clone, Copy, Debug)]
pub struct FailedDevelopmentEntitiesPullRequests;

impl FailedDevelopmentEntitiesPullRequests {
    pub const ERROR_MESSAGES: crate::Field<
        Self,
        crate::List<crate::Object<super::FailedDevelopmentEntitiesPullRequestsErrorMessages>>,
    > = crate::Field::new("errorMessages");

    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
}
