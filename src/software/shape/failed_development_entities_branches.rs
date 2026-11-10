#[derive(Clone, Copy, Debug)]
pub struct FailedDevelopmentEntitiesBranches;

impl FailedDevelopmentEntitiesBranches {
    pub const ERROR_MESSAGES: crate::Field<
        Self,
        crate::List<crate::Object<super::FailedDevelopmentEntitiesBranchesErrorMessages>>,
    > = crate::Field::new("errorMessages");

    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
}
