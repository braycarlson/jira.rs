#[derive(Clone, Copy, Debug)]
pub struct VersionUsageInCustomField;

impl VersionUsageInCustomField {
    pub const CUSTOM_FIELD_ID: crate::Field<Self, crate::Integer> =
        crate::Field::new("customFieldId");

    pub const FIELD_NAME: crate::Field<Self, crate::Text> = crate::Field::new("fieldName");

    pub const ISSUE_WITH_VERSION_IN_CUSTOM_FIELD_COUNT: crate::Field<Self, crate::Integer> =
        crate::Field::new("issueCountWithVersionInCustomField");
}
