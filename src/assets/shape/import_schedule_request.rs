#[derive(Clone, Copy, Debug)]
pub struct ImportScheduleRequest;

impl ImportScheduleRequest {
    pub const CALLBACK_URL: crate::Field<Self, crate::Text> = crate::Field::new("callbackUrl");
    pub const RUN_INTERVAL: crate::Field<Self, crate::Text> = crate::Field::new("runInterval");
    pub const START_TIME: crate::Field<Self, crate::Text> = crate::Field::new("startTime");
}
