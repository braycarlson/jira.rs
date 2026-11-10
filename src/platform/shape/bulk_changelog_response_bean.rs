#[derive(Clone, Copy, Debug)]
pub struct BulkChangelogResponseBean;

impl BulkChangelogResponseBean {
    pub const ISSUE_CHANGE_LOGS: crate::Field<
        Self,
        crate::List<crate::Object<super::IssueChangeLog>>,
    > = crate::Field::new("issueChangeLogs");

    pub const PAGE_TOKEN_NEXT: crate::Field<Self, crate::Text> = crate::Field::new("nextPageToken");
}
