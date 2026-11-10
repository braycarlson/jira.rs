#[derive(Clone, Copy, Debug)]
pub struct SearchAndReconcileRequestBean;

impl SearchAndReconcileRequestBean {
    pub const EXPAND: crate::Field<Self, crate::Text> = crate::Field::new("expand");
    pub const FIELDS: crate::Field<Self, crate::List<crate::Text>> = crate::Field::new("fields");

    pub const FIELDS_BY_KEYS: crate::Field<Self, crate::Boolean> =
        crate::Field::new("fieldsByKeys");

    pub const INCLUDE_ARCHIVED_PROJECTS: crate::Field<Self, crate::Boolean> =
        crate::Field::new("includeArchivedProjects");

    pub const JQL: crate::Field<Self, crate::Text> = crate::Field::new("jql");
    pub const PAGE_TOKEN_NEXT: crate::Field<Self, crate::Text> = crate::Field::new("nextPageToken");

    pub const PROPERTIES: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("properties");

    pub const RECONCILE_ISSUES: crate::Field<Self, crate::List<crate::Integer>> =
        crate::Field::new("reconcileIssues");

    pub const RESULTS_MAX: crate::Field<Self, crate::Integer> = crate::Field::new("maxResults");
}
