#[derive(Clone, Copy, Debug)]
pub struct GetConfigurationReplyColumnConfigColumnsStatuses;

impl GetConfigurationReplyColumnConfigColumnsStatuses {
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const SELF: crate::Field<Self, crate::Text> = crate::Field::new("self");
}
