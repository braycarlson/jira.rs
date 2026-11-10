#[derive(Clone, Copy, Debug)]
pub struct SubmitEntityReply;

impl SubmitEntityReply {
    pub const ACCEPTED_INCIDENTS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("acceptedIncidents");

    pub const FAILED_INCIDENTS: crate::Field<
        Self,
        crate::Map<crate::List<crate::Object<super::SubmitEntityReplyFailedIncidents>>>,
    > = crate::Field::new("failedIncidents");

    pub const UNKNOWN_PROJECT_KEYS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("unknownProjectKeys");
}
