#[derive(Clone, Copy, Debug)]
pub struct SanitizedJQLQueries;

impl SanitizedJQLQueries {
    pub const QUERIES: crate::Field<Self, crate::List<crate::Object<super::SanitizedJQLQuery>>> =
        crate::Field::new("queries");
}
