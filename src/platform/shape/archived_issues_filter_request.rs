#[derive(Clone, Copy, Debug)]
pub struct ArchivedIssuesFilterRequest;

impl ArchivedIssuesFilterRequest {
    pub const ARCHIVED_BY: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("archivedBy");

    pub const ARCHIVED_DATE_RANGE: crate::Field<
        Self,
        crate::Object<super::DateRangeFilterRequest>,
    > = crate::Field::new("archivedDateRange");

    pub const ISSUE_TYPES: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("issueTypes");

    pub const PROJECTS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("projects");

    pub const REPORTERS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("reporters");
}
