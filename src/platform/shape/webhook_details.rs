#[derive(Clone, Copy, Debug)]
pub struct WebhookDetails;

impl WebhookDetails {
    pub const EVENTS: crate::Field<Self, crate::List<crate::Text>> = crate::Field::new("events");

    pub const FIELD_IDS_FILTER: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("fieldIdsFilter");

    pub const ISSUE_PROPERTY_KEYS_FILTER: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("issuePropertyKeysFilter");

    pub const JQL_FILTER: crate::Field<Self, crate::Text> = crate::Field::new("jqlFilter");
}
