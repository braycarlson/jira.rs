#[derive(Clone, Copy, Debug)]
pub struct GetPlanResponse;

impl GetPlanResponse {
    pub const CROSS_PROJECT_RELEASES: crate::Field<
        Self,
        crate::List<crate::Object<super::GetCrossProjectReleaseResponse>>,
    > = crate::Field::new("crossProjectReleases");

    pub const CUSTOM_FIELDS: crate::Field<
        Self,
        crate::List<crate::Object<super::GetCustomFieldResponse>>,
    > = crate::Field::new("customFields");

    pub const EXCLUSION_RULES: crate::Field<Self, crate::Object<super::GetExclusionRulesResponse>> =
        crate::Field::new("exclusionRules");

    pub const ID: crate::Field<Self, crate::Integer> = crate::Field::new("id");

    pub const ISSUE_SOURCES: crate::Field<
        Self,
        crate::List<crate::Object<super::GetIssueSourceResponse>>,
    > = crate::Field::new("issueSources");

    pub const LEAD_ACCOUNT_ID: crate::Field<Self, crate::Text> = crate::Field::new("leadAccountId");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");

    pub const PERMISSIONS: crate::Field<
        Self,
        crate::List<crate::Object<super::GetPermissionResponse>>,
    > = crate::Field::new("permissions");

    pub const SAVED_LAST: crate::Field<Self, crate::Text> = crate::Field::new("lastSaved");

    pub const SCHEDULING: crate::Field<Self, crate::Object<super::GetSchedulingResponse>> =
        crate::Field::new("scheduling");

    pub const STATUS: crate::Field<Self, crate::Text> = crate::Field::new("status");
}
