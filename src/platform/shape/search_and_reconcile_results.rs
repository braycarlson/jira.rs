#[derive(Clone, Copy, Debug)]
pub struct SearchAndReconcileResults;

impl SearchAndReconcileResults {
    pub const ISSUES: crate::Field<Self, crate::List<crate::Object<super::IssueBean>>> =
        crate::Field::new("issues");

    pub const IS_LAST: crate::Field<Self, crate::Boolean> = crate::Field::new("isLast");
    pub const NAMES: crate::Field<Self, crate::Map<crate::Text>> = crate::Field::new("names");
    pub const PAGE_TOKEN_NEXT: crate::Field<Self, crate::Text> = crate::Field::new("nextPageToken");

    pub const SCHEMA: crate::Field<Self, crate::Map<crate::Object<super::JSONTypeBean>>> =
        crate::Field::new("schema");

    pub const WARNINGS: crate::Field<Self, crate::List<crate::Object<super::SearchWarning>>> =
        crate::Field::new("warnings");
}
