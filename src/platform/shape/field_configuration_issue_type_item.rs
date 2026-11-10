#[derive(Clone, Copy, Debug)]
pub struct FieldConfigurationIssueTypeItem;

impl FieldConfigurationIssueTypeItem {
    pub const FIELD_CONFIGURATION_ID: crate::Field<Self, crate::Text> =
        crate::Field::new("fieldConfigurationId");

    pub const FIELD_CONFIGURATION_SCHEME_ID: crate::Field<Self, crate::Text> =
        crate::Field::new("fieldConfigurationSchemeId");

    pub const ISSUE_TYPE_ID: crate::Field<Self, crate::Text> = crate::Field::new("issueTypeId");
}
