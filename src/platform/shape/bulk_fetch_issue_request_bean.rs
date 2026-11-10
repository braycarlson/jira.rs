#[derive(Clone, Copy, Debug)]
pub struct BulkFetchIssueRequestBean;

impl BulkFetchIssueRequestBean {
    pub const EXPAND: crate::Field<Self, crate::List<crate::Text>> = crate::Field::new("expand");
    pub const FIELDS: crate::Field<Self, crate::List<crate::Text>> = crate::Field::new("fields");

    pub const FIELDS_BY_KEYS: crate::Field<Self, crate::Boolean> =
        crate::Field::new("fieldsByKeys");

    pub const ISSUE_IDS_OR_KEYS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("issueIdsOrKeys");

    pub const PROPERTIES: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("properties");
}
