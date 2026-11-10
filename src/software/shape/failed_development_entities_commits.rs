#[derive(Clone, Copy, Debug)]
pub struct FailedDevelopmentEntitiesCommits;

impl FailedDevelopmentEntitiesCommits {
    pub const ERROR_MESSAGES: crate::Field<
        Self,
        crate::List<crate::Object<super::FailedDevelopmentEntitiesCommitsErrorMessages>>,
    > = crate::Field::new("errorMessages");

    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
}
