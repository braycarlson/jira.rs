#[derive(Clone, Copy, Debug)]
pub struct SearchRequestBean;

impl SearchRequestBean {
    pub const EXPAND: crate::Field<Self, crate::List<crate::Text>> = crate::Field::new("expand");
    pub const FIELDS: crate::Field<Self, crate::List<crate::Text>> = crate::Field::new("fields");

    pub const FIELDS_BY_KEYS: crate::Field<Self, crate::Boolean> =
        crate::Field::new("fieldsByKeys");

    pub const JQL: crate::Field<Self, crate::Text> = crate::Field::new("jql");

    pub const PROPERTIES: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("properties");

    pub const RESULTS_MAX: crate::Field<Self, crate::Integer> = crate::Field::new("maxResults");
    pub const START_AT: crate::Field<Self, crate::Integer> = crate::Field::new("startAt");
    pub const VALIDATE_QUERY: crate::Field<Self, crate::Text> = crate::Field::new("validateQuery");
}
