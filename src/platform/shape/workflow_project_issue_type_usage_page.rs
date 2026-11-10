#[derive(Clone, Copy, Debug)]
pub struct WorkflowProjectIssueTypeUsagePage;

impl WorkflowProjectIssueTypeUsagePage {
    pub const PAGE_TOKEN_NEXT: crate::Field<Self, crate::Text> = crate::Field::new("nextPageToken");

    pub const VALUES: crate::Field<
        Self,
        crate::List<crate::Object<super::WorkflowProjectIssueTypeUsage>>,
    > = crate::Field::new("values");
}
