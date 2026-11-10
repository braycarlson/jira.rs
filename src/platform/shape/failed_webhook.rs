#[derive(Clone, Copy, Debug)]
pub struct FailedWebhook;

impl FailedWebhook {
    pub const BODY: crate::Field<Self, crate::Text> = crate::Field::new("body");
    pub const FAILURE_TIME: crate::Field<Self, crate::Integer> = crate::Field::new("failureTime");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const URL: crate::Field<Self, crate::Text> = crate::Field::new("url");
}
