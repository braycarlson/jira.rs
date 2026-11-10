#[derive(Clone, Copy, Debug)]
pub struct SanitizedJQLQuery;

impl SanitizedJQLQuery {
    pub const ACCOUNT_ID: crate::Field<Self, crate::Text> = crate::Field::new("accountId");

    pub const ERRORS: crate::Field<Self, crate::Object<super::ErrorCollection>> =
        crate::Field::new("errors");

    pub const QUERY_INITIAL: crate::Field<Self, crate::Text> = crate::Field::new("initialQuery");

    pub const SANITIZED_QUERY: crate::Field<Self, crate::Text> =
        crate::Field::new("sanitizedQuery");
}
