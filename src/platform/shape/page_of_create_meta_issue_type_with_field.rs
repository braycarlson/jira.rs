#[derive(Clone, Copy, Debug)]
pub struct PageOfCreateMetaIssueTypeWithField;

impl PageOfCreateMetaIssueTypeWithField {
    pub const FIELDS: crate::Field<Self, crate::List<crate::Object<super::FieldCreateMetadata>>> =
        crate::Field::new("fields");

    pub const RESULTS: crate::Field<Self, crate::List<crate::Object<super::FieldCreateMetadata>>> =
        crate::Field::new("results");

    pub const RESULTS_MAX: crate::Field<Self, crate::Integer> = crate::Field::new("maxResults");
    pub const START_AT: crate::Field<Self, crate::Integer> = crate::Field::new("startAt");
    pub const TOTAL: crate::Field<Self, crate::Integer> = crate::Field::new("total");
}
