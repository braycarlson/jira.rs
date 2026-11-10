#[derive(Clone, Copy, Debug)]
pub struct PageOfChangelogs;

impl PageOfChangelogs {
    pub const HISTORIES: crate::Field<
        Self,
        crate::List<crate::Object<super::PageOfChangelogsHistories>>,
    > = crate::Field::new("histories");

    pub const RESULTS_MAX: crate::Field<Self, crate::Integer> = crate::Field::new("maxResults");
    pub const START_AT: crate::Field<Self, crate::Integer> = crate::Field::new("startAt");
    pub const TOTAL: crate::Field<Self, crate::Integer> = crate::Field::new("total");
}
