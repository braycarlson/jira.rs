#[derive(Clone, Copy, Debug)]
pub struct AuditPage;

impl AuditPage {
    pub const ENTRIES: crate::Field<Self, crate::List<crate::Object<super::AuditEntry>>> =
        crate::Field::new("entries");

    pub const IS_LAST: crate::Field<Self, crate::Boolean> = crate::Field::new("isLast");
    pub const PAGE_TOKEN_NEXT: crate::Field<Self, crate::Text> = crate::Field::new("nextPageToken");
    pub const PAGE_URL_NEXT: crate::Field<Self, crate::Text> = crate::Field::new("nextPageURL");
}
