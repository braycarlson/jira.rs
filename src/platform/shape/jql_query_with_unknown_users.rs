#[derive(Clone, Copy, Debug)]
pub struct JQLQueryWithUnknownUsers;

impl JQLQueryWithUnknownUsers {
    pub const CONVERTED_QUERY: crate::Field<Self, crate::Text> =
        crate::Field::new("convertedQuery");

    pub const ORIGINAL_QUERY: crate::Field<Self, crate::Text> = crate::Field::new("originalQuery");
}
