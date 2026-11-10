#[derive(Clone, Copy, Debug)]
pub struct DateDTO;

impl DateDTO {
    pub const EPOCH_MILLIS: crate::Field<Self, crate::Integer> = crate::Field::new("epochMillis");
    pub const FRIENDLY: crate::Field<Self, crate::Text> = crate::Field::new("friendly");
    pub const ISO8601: crate::Field<Self, crate::Text> = crate::Field::new("iso8601");
    pub const JIRA: crate::Field<Self, crate::Text> = crate::Field::new("jira");
}
