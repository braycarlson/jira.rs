#[derive(Clone, Copy, Debug)]
pub struct BulkIssueIsWatching;

impl BulkIssueIsWatching {
    pub const ISSUES_IS_WATCHING: crate::Field<Self, crate::Map<crate::Boolean>> =
        crate::Field::new("issuesIsWatching");
}
