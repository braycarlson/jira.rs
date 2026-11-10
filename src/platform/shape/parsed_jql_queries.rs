#[derive(Clone, Copy, Debug)]
pub struct ParsedJQLQueries;

impl ParsedJQLQueries {
    pub const QUERIES: crate::Field<Self, crate::List<crate::Object<super::ParsedJQLQuery>>> =
        crate::Field::new("queries");
}
