#[derive(Clone, Copy, Debug)]
pub struct TimeTrackingProvider;

impl TimeTrackingProvider {
    pub const KEY: crate::Field<Self, crate::Text> = crate::Field::new("key");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
    pub const URL: crate::Field<Self, crate::Text> = crate::Field::new("url");
}
