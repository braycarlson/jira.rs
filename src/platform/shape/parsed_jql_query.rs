#[derive(Clone, Copy, Debug)]
pub struct ParsedJQLQuery;

impl ParsedJQLQuery {
    pub const ERRORS: crate::Field<Self, crate::List<crate::Text>> = crate::Field::new("errors");
    pub const QUERY: crate::Field<Self, crate::Text> = crate::Field::new("query");

    pub const STRUCTURE: crate::Field<Self, crate::Object<super::JQLQuery>> =
        crate::Field::new("structure");

    pub const WARNINGS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("warnings");
}
