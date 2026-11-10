#[derive(Clone, Copy, Debug)]
pub struct BulkChangelogRequestBean;

impl BulkChangelogRequestBean {
    pub const FIELD_IDS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("fieldIds");

    pub const ISSUE_IDS_OR_KEYS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("issueIdsOrKeys");

    pub const PAGE_TOKEN_NEXT: crate::Field<Self, crate::Text> = crate::Field::new("nextPageToken");
    pub const RESULTS_MAX: crate::Field<Self, crate::Integer> = crate::Field::new("maxResults");
}
