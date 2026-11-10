#[derive(Clone, Copy, Debug)]
pub struct SubmitComponentsReply;

impl SubmitComponentsReply {
    pub const ACCEPTED_COMPONENTS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("acceptedComponents");

    pub const FAILED_COMPONENTS: crate::Field<
        Self,
        crate::Map<crate::List<crate::Object<super::SubmitComponentsReplyFailedComponents>>>,
    > = crate::Field::new("failedComponents");

    pub const UNKNOWN_PROJECT_KEYS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("unknownProjectKeys");
}
