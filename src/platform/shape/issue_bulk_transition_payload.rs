#[derive(Clone, Copy, Debug)]
pub struct IssueBulkTransitionPayload;

impl IssueBulkTransitionPayload {
    pub const BULK_TRANSITION_INPUTS: crate::Field<
        Self,
        crate::List<crate::Object<super::BulkTransitionSubmitInput>>,
    > = crate::Field::new("bulkTransitionInputs");

    pub const SEND_BULK_NOTIFICATION: crate::Field<Self, crate::Boolean> =
        crate::Field::new("sendBulkNotification");
}
