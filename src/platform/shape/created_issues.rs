#[derive(Clone, Copy, Debug)]
pub struct CreatedIssues;

impl CreatedIssues {
    pub const ERRORS: crate::Field<
        Self,
        crate::List<crate::Object<super::BulkOperationErrorResult>>,
    > = crate::Field::new("errors");

    pub const ISSUES: crate::Field<Self, crate::List<crate::Object<super::CreatedIssue>>> =
        crate::Field::new("issues");
}
