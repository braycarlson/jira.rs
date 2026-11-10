#[derive(Clone, Copy, Debug)]
pub struct ProjectInsight;

impl ProjectInsight {
    pub const ISSUE_COUNT_TOTAL: crate::Field<Self, crate::Integer> =
        crate::Field::new("totalIssueCount");

    pub const ISSUE_UPDATE_TIME_LAST: crate::Field<Self, crate::Text> =
        crate::Field::new("lastIssueUpdateTime");
}
