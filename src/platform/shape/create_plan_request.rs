#[derive(Clone, Copy, Debug)]
pub struct CreatePlanRequest;

impl CreatePlanRequest {
    pub const CROSS_PROJECT_RELEASES: crate::Field<
        Self,
        crate::List<crate::Object<super::CreateCrossProjectReleaseRequest>>,
    > = crate::Field::new("crossProjectReleases");

    pub const CUSTOM_FIELDS: crate::Field<
        Self,
        crate::List<crate::Object<super::CreateCustomFieldRequest>>,
    > = crate::Field::new("customFields");

    pub const EXCLUSION_RULES: crate::Field<
        Self,
        crate::Object<super::CreateExclusionRulesRequest>,
    > = crate::Field::new("exclusionRules");

    pub const ISSUE_SOURCES: crate::Field<
        Self,
        crate::List<crate::Object<super::CreateIssueSourceRequest>>,
    > = crate::Field::new("issueSources");

    pub const LEAD_ACCOUNT_ID: crate::Field<Self, crate::Text> = crate::Field::new("leadAccountId");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");

    pub const PERMISSIONS: crate::Field<
        Self,
        crate::List<crate::Object<super::CreatePermissionRequest>>,
    > = crate::Field::new("permissions");

    pub const SCHEDULING: crate::Field<Self, crate::Object<super::CreateSchedulingRequest>> =
        crate::Field::new("scheduling");
}
