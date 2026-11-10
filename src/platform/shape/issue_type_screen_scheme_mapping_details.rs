#[derive(Clone, Copy, Debug)]
pub struct IssueTypeScreenSchemeMappingDetails;

impl IssueTypeScreenSchemeMappingDetails {
    pub const ISSUE_TYPE_MAPPINGS: crate::Field<
        Self,
        crate::List<crate::Object<super::IssueTypeScreenSchemeMapping>>,
    > = crate::Field::new("issueTypeMappings");
}
