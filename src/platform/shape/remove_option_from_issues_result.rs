#[derive(Clone, Copy, Debug)]
pub struct RemoveOptionFromIssuesResult;

impl RemoveOptionFromIssuesResult {
    pub const ERRORS: crate::Field<Self, crate::Object<super::SimpleErrorCollection>> =
        crate::Field::new("errors");

    pub const MODIFIED_ISSUES: crate::Field<Self, crate::List<crate::Integer>> =
        crate::Field::new("modifiedIssues");

    pub const UNMODIFIED_ISSUES: crate::Field<Self, crate::List<crate::Integer>> =
        crate::Field::new("unmodifiedIssues");
}
