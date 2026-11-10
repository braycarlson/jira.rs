#[derive(Clone, Copy, Debug)]
pub struct IssueTypeDefaultValueUpdate;

impl IssueTypeDefaultValueUpdate {
    pub const CONTEXT_ID: crate::Field<Self, crate::Integer> = crate::Field::new("contextId");
    pub const ISSUE_TYPE_ID: crate::Field<Self, crate::Text> = crate::Field::new("issueTypeId");

    pub const IS_ANY_ISSUE_TYPE: crate::Field<Self, crate::Boolean> =
        crate::Field::new("isAnyIssueType");

    pub const VALUE: crate::Field<Self, crate::Any> = crate::Field::new("value");
}
