#[derive(Clone, Copy, Debug)]
pub struct CreateCrossProjectReleaseRequest;

impl CreateCrossProjectReleaseRequest {
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");

    pub const RELEASE_IDS: crate::Field<Self, crate::List<crate::Integer>> =
        crate::Field::new("releaseIds");
}
