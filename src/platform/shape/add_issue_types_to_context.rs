#[derive(Clone, Copy, Debug)]
pub struct AddIssueTypesToContext;

impl AddIssueTypesToContext {
    pub const DEFAULT_VALUE: crate::Field<Self, crate::Any> = crate::Field::new("defaultValue");

    pub const ISSUE_TYPE_IDS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("issueTypeIds");

    pub const IS_ANY_ISSUE_TYPE: crate::Field<Self, crate::Boolean> =
        crate::Field::new("isAnyIssueType");
}
