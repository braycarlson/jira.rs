#[derive(Clone, Copy, Debug)]
pub struct JexpEvaluateCtxJQLIssues;

impl JexpEvaluateCtxJQLIssues {
    pub const PAGE_TOKEN_NEXT: crate::Field<Self, crate::Text> = crate::Field::new("nextPageToken");
    pub const QUERY: crate::Field<Self, crate::Text> = crate::Field::new("query");
    pub const RESULTS_MAX: crate::Field<Self, crate::Integer> = crate::Field::new("maxResults");
}
