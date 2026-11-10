#[derive(Clone, Copy, Debug)]
pub struct ScheduledImportDetails;

impl ScheduledImportDetails {
    pub const CREATED_AT: crate::Field<Self, crate::Text> = crate::Field::new("createdAt");

    pub const IMPORT_SCHEDULE_ID: crate::Field<Self, crate::Text> =
        crate::Field::new("importScheduleId");

    pub const RUN_FREQUENCY: crate::Field<Self, crate::Text> = crate::Field::new("runFrequency");

    pub const SCHEDULED_TIME_NEXT: crate::Field<Self, crate::Text> =
        crate::Field::new("nextScheduledTime");

    pub const START_TIME: crate::Field<Self, crate::Text> = crate::Field::new("startTime");
}
