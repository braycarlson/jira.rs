#[derive(Clone, Copy, Debug)]
pub struct StatusWorkflowUsagePage;

impl StatusWorkflowUsagePage {
    pub const PAGE_TOKEN_NEXT: crate::Field<Self, crate::Text> = crate::Field::new("nextPageToken");

    pub const VALUES: crate::Field<
        Self,
        crate::List<crate::Object<super::StatusWorkflowUsageWorkflow>>,
    > = crate::Field::new("values");
}
