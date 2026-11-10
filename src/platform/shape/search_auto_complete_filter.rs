#[derive(Clone, Copy, Debug)]
pub struct SearchAutoCompleteFilter;

impl SearchAutoCompleteFilter {
    pub const INCLUDE_COLLAPSED_FIELDS: crate::Field<Self, crate::Boolean> =
        crate::Field::new("includeCollapsedFields");

    pub const PROJECT_IDS: crate::Field<Self, crate::List<crate::Integer>> =
        crate::Field::new("projectIds");
}
