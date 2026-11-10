#[derive(Clone, Copy, Debug)]
pub struct SubmitBuildsReplyRejectedBuilds;

impl SubmitBuildsReplyRejectedBuilds {
    pub const ERRORS: crate::Field<
        Self,
        crate::List<crate::Object<super::SubmitBuildsReplyRejectedBuildsErrors>>,
    > = crate::Field::new("errors");

    pub const KEY: crate::Field<Self, crate::Object<super::SubmitBuildsReplyRejectedBuildsKey>> =
        crate::Field::new("key");
}
