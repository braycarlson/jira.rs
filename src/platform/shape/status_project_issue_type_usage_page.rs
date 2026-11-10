#[derive(Clone, Copy, Debug)]
pub struct StatusProjectIssueTypeUsagePage;

impl StatusProjectIssueTypeUsagePage {
    pub const PAGE_TOKEN_NEXT: crate::Field<Self, crate::Text> = crate::Field::new("nextPageToken");

    pub const VALUES: crate::Field<
        Self,
        crate::List<crate::Object<super::StatusProjectIssueTypeUsage>>,
    > = crate::Field::new("values");
}
