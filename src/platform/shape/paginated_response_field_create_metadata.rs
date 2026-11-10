#[derive(Clone, Copy, Debug)]
pub struct PaginatedResponseFieldCreateMetadata;

impl PaginatedResponseFieldCreateMetadata {
    pub const RESULTS: crate::Field<Self, crate::List<crate::Object<super::FieldCreateMetadata>>> =
        crate::Field::new("results");

    pub const RESULTS_MAX: crate::Field<Self, crate::Integer> = crate::Field::new("maxResults");
    pub const START_AT: crate::Field<Self, crate::Integer> = crate::Field::new("startAt");
    pub const TOTAL: crate::Field<Self, crate::Integer> = crate::Field::new("total");
}
