#[derive(Clone, Copy, Debug)]
pub struct FieldConfigurationToIssueTypeMapping;

impl FieldConfigurationToIssueTypeMapping {
    pub const FIELD_CONFIGURATION_ID: crate::Field<Self, crate::Text> =
        crate::Field::new("fieldConfigurationId");

    pub const ISSUE_TYPE_ID: crate::Field<Self, crate::Text> = crate::Field::new("issueTypeId");
}
