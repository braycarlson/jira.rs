#[derive(Clone, Copy, Debug)]
pub struct Webhook;

impl Webhook {
    pub const EVENTS: crate::Field<Self, crate::List<crate::Text>> = crate::Field::new("events");
    pub const EXPIRATION_DATE: crate::Field<Self, crate::Any> = crate::Field::new("expirationDate");

    pub const FIELD_IDS_FILTER: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("fieldIdsFilter");

    pub const ID: crate::Field<Self, crate::Integer> = crate::Field::new("id");

    pub const ISSUE_PROPERTY_KEYS_FILTER: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("issuePropertyKeysFilter");

    pub const JQL_FILTER: crate::Field<Self, crate::Text> = crate::Field::new("jqlFilter");
    pub const URL: crate::Field<Self, crate::Text> = crate::Field::new("url");
}
