#[derive(Clone, Copy, Debug)]
pub struct JQLQueriesToSanitize;

impl JQLQueriesToSanitize {
    pub const QUERIES: crate::Field<Self, crate::List<crate::Object<super::JQLQueryToSanitize>>> =
        crate::Field::new("queries");
}
