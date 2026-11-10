#[derive(Clone, Copy, Debug)]
pub struct IssueBulkMovePayload;

impl IssueBulkMovePayload {
    pub const SEND_BULK_NOTIFICATION: crate::Field<Self, crate::Boolean> =
        crate::Field::new("sendBulkNotification");

    pub const TARGET_TO_SOURCES_MAPPING: crate::Field<
        Self,
        crate::Map<crate::Object<super::TargetToSourcesMapping>>,
    > = crate::Field::new("targetToSourcesMapping");
}
