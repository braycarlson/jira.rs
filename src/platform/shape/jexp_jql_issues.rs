#[derive(Clone, Copy, Debug)]
pub struct JexpJQLIssues;

impl JexpJQLIssues {
    pub const QUERY: crate::Field<Self, crate::Text> = crate::Field::new("query");
    pub const RESULTS_MAX: crate::Field<Self, crate::Integer> = crate::Field::new("maxResults");
    pub const START_AT: crate::Field<Self, crate::Integer> = crate::Field::new("startAt");
    pub const VALIDATION: crate::Field<Self, crate::Text> = crate::Field::new("validation");
}
