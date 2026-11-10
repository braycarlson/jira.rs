#[derive(Clone, Copy, Debug)]
pub struct GetImportScheduleLinksReplyLinks;

impl GetImportScheduleLinksReplyLinks {
    pub const CREATE_SCHEDULE: crate::Field<Self, crate::Text> =
        crate::Field::new("createSchedule");

    pub const SCHEDULE: crate::Field<Self, crate::Text> = crate::Field::new("schedule");
}
