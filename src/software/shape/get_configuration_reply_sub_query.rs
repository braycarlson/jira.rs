#[derive(Clone, Copy, Debug)]
pub struct GetConfigurationReplySubQuery;

impl GetConfigurationReplySubQuery {
    pub const QUERY: crate::Field<Self, crate::Text> = crate::Field::new("query");
}
