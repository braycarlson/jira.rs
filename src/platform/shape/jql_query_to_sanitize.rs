#[derive(Clone, Copy, Debug)]
pub struct JQLQueryToSanitize;

impl JQLQueryToSanitize {
    pub const ACCOUNT_ID: crate::Field<Self, crate::Text> = crate::Field::new("accountId");
    pub const QUERY: crate::Field<Self, crate::Text> = crate::Field::new("query");
}
