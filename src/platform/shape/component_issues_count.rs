#[derive(Clone, Copy, Debug)]
pub struct ComponentIssuesCount;

impl ComponentIssuesCount {
    pub const ISSUE_COUNT: crate::Field<Self, crate::Integer> = crate::Field::new("issueCount");
    pub const SELF: crate::Field<Self, crate::Text> = crate::Field::new("self");
}
