#[derive(Clone, Copy, Debug)]
pub struct WorkflowSchemeUsagePage;

impl WorkflowSchemeUsagePage {
    pub const PAGE_TOKEN_NEXT: crate::Field<Self, crate::Text> = crate::Field::new("nextPageToken");

    pub const VALUES: crate::Field<Self, crate::List<crate::Object<super::WorkflowSchemeUsage>>> =
        crate::Field::new("values");
}
