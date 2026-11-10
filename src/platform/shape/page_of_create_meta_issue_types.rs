#[derive(Clone, Copy, Debug)]
pub struct PageOfCreateMetaIssueTypes;

impl PageOfCreateMetaIssueTypes {
    pub const CREATE_META_ISSUE_TYPE: crate::Field<
        Self,
        crate::List<crate::Object<super::IssueTypeIssueCreateMetadata>>,
    > = crate::Field::new("createMetaIssueType");

    pub const ISSUE_TYPES: crate::Field<
        Self,
        crate::List<crate::Object<super::IssueTypeIssueCreateMetadata>>,
    > = crate::Field::new("issueTypes");

    pub const RESULTS_MAX: crate::Field<Self, crate::Integer> = crate::Field::new("maxResults");
    pub const START_AT: crate::Field<Self, crate::Integer> = crate::Field::new("startAt");
    pub const TOTAL: crate::Field<Self, crate::Integer> = crate::Field::new("total");
}
