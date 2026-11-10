#[derive(Clone, Copy, Debug)]
pub struct FieldAssociationSchemeMatchedFilters;

impl FieldAssociationSchemeMatchedFilters {
    pub const PROJECT_IDS: crate::Field<Self, crate::List<crate::Integer>> =
        crate::Field::new("projectIds");

    pub const QUERY: crate::Field<Self, crate::Text> = crate::Field::new("query");
}
