#[derive(Clone, Copy, Debug)]
pub struct StatusProjectUsagePage;

impl StatusProjectUsagePage {
    pub const PAGE_TOKEN_NEXT: crate::Field<Self, crate::Text> = crate::Field::new("nextPageToken");

    pub const VALUES: crate::Field<Self, crate::List<crate::Object<super::StatusProjectUsage>>> =
        crate::Field::new("values");
}
