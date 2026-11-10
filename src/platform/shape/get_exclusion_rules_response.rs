#[derive(Clone, Copy, Debug)]
pub struct GetExclusionRulesResponse;

impl GetExclusionRulesResponse {
    pub const ISSUE_IDS: crate::Field<Self, crate::List<crate::Integer>> =
        crate::Field::new("issueIds");

    pub const ISSUE_TYPE_IDS: crate::Field<Self, crate::List<crate::Integer>> =
        crate::Field::new("issueTypeIds");

    pub const NUMBER_OF_DAYS_TO_SHOW_COMPLETED_ISSUES: crate::Field<Self, crate::Integer> =
        crate::Field::new("numberOfDaysToShowCompletedIssues");

    pub const RELEASE_IDS: crate::Field<Self, crate::List<crate::Integer>> =
        crate::Field::new("releaseIds");

    pub const WORK_STATUS_CATEGORY_IDS: crate::Field<Self, crate::List<crate::Integer>> =
        crate::Field::new("workStatusCategoryIds");

    pub const WORK_STATUS_IDS: crate::Field<Self, crate::List<crate::Integer>> =
        crate::Field::new("workStatusIds");
}
