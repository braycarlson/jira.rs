#[derive(Clone, Copy, Debug)]
pub struct WorkflowSchemeProjectSwitchBean;

impl WorkflowSchemeProjectSwitchBean {
    pub const MAPPINGS_BY_ISSUE_TYPE_OVERRIDE: crate::Field<
        Self,
        crate::List<crate::Object<super::MappingsByIssueTypeOverride>>,
    > = crate::Field::new("mappingsByIssueTypeOverride");

    pub const PROJECT_ID: crate::Field<Self, crate::Text> = crate::Field::new("projectId");

    pub const TARGET_SCHEME_ID: crate::Field<Self, crate::Text> =
        crate::Field::new("targetSchemeId");
}
