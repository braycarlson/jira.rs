#[derive(Clone, Copy, Debug)]
pub struct ProjectUsagePage;

impl ProjectUsagePage {
    pub const PAGE_TOKEN_NEXT: crate::Field<Self, crate::Text> = crate::Field::new("nextPageToken");

    pub const VALUES: crate::Field<Self, crate::List<crate::Object<super::ProjectUsage>>> =
        crate::Field::new("values");
}
