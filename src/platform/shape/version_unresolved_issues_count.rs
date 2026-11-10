#[derive(Clone, Copy, Debug)]
pub struct VersionUnresolvedIssuesCount;

impl VersionUnresolvedIssuesCount {
    pub const ISSUES_COUNT: crate::Field<Self, crate::Integer> = crate::Field::new("issuesCount");

    pub const ISSUES_UNRESOLVED_COUNT: crate::Field<Self, crate::Integer> =
        crate::Field::new("issuesUnresolvedCount");

    pub const SELF: crate::Field<Self, crate::Text> = crate::Field::new("self");
}
