#[derive(Clone, Copy, Debug)]
pub struct ConvertedJQLQueries;

impl ConvertedJQLQueries {
    pub const QUERIES_WITH_UNKNOWN_USERS: crate::Field<
        Self,
        crate::List<crate::Object<super::JQLQueryWithUnknownUsers>>,
    > = crate::Field::new("queriesWithUnknownUsers");

    pub const QUERY_STRINGS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("queryStrings");
}
