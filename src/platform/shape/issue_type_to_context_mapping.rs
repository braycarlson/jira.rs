#[derive(Clone, Copy, Debug)]
pub struct IssueTypeToContextMapping;

impl IssueTypeToContextMapping {
    pub const CONTEXT_ID: crate::Field<Self, crate::Text> = crate::Field::new("contextId");
    pub const ISSUE_TYPE_ID: crate::Field<Self, crate::Text> = crate::Field::new("issueTypeId");

    pub const IS_ANY_ISSUE_TYPE: crate::Field<Self, crate::Boolean> =
        crate::Field::new("isAnyIssueType");
}
