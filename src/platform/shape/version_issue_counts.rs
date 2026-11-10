#[derive(Clone, Copy, Debug)]
pub struct VersionIssueCounts;

impl VersionIssueCounts {
    pub const CUSTOM_FIELD_USAGE: crate::Field<
        Self,
        crate::List<crate::Object<super::VersionUsageInCustomField>>,
    > = crate::Field::new("customFieldUsage");

    pub const ISSUES_AFFECTED_COUNT: crate::Field<Self, crate::Integer> =
        crate::Field::new("issuesAffectedCount");

    pub const ISSUES_FIXED_COUNT: crate::Field<Self, crate::Integer> =
        crate::Field::new("issuesFixedCount");

    pub const ISSUE_WITH_CUSTOM_FIELDS_SHOWING_VERSION_COUNT: crate::Field<Self, crate::Integer> =
        crate::Field::new("issueCountWithCustomFieldsShowingVersion");

    pub const SELF: crate::Field<Self, crate::Text> = crate::Field::new("self");
}
