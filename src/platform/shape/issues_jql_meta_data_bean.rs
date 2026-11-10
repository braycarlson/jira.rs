#[derive(Clone, Copy, Debug)]
pub struct IssuesJQLMetaDataBean;

impl IssuesJQLMetaDataBean {
    pub const COUNT: crate::Field<Self, crate::Integer> = crate::Field::new("count");
    pub const COUNT_TOTAL: crate::Field<Self, crate::Integer> = crate::Field::new("totalCount");
    pub const RESULTS_MAX: crate::Field<Self, crate::Integer> = crate::Field::new("maxResults");
    pub const START_AT: crate::Field<Self, crate::Integer> = crate::Field::new("startAt");

    pub const VALIDATION_WARNINGS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("validationWarnings");
}
