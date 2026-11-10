#[derive(Clone, Copy, Debug)]
pub struct SearchResults;

impl SearchResults {
    pub const EXPAND: crate::Field<Self, crate::Text> = crate::Field::new("expand");

    pub const ISSUES: crate::Field<Self, crate::List<crate::Object<super::IssueBean>>> =
        crate::Field::new("issues");

    pub const NAMES: crate::Field<Self, crate::Map<crate::Text>> = crate::Field::new("names");
    pub const RESULTS_MAX: crate::Field<Self, crate::Integer> = crate::Field::new("maxResults");

    pub const SCHEMA: crate::Field<Self, crate::Map<crate::Object<super::SearchResultsSchema>>> =
        crate::Field::new("schema");

    pub const START_AT: crate::Field<Self, crate::Integer> = crate::Field::new("startAt");
    pub const TOTAL: crate::Field<Self, crate::Integer> = crate::Field::new("total");

    pub const WARNING_MESSAGES: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("warningMessages");
}
