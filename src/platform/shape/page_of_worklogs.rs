#[derive(Clone, Copy, Debug)]
pub struct PageOfWorklogs;

impl PageOfWorklogs {
    pub const RESULTS_MAX: crate::Field<Self, crate::Integer> = crate::Field::new("maxResults");
    pub const START_AT: crate::Field<Self, crate::Integer> = crate::Field::new("startAt");
    pub const TOTAL: crate::Field<Self, crate::Integer> = crate::Field::new("total");

    pub const WORKLOGS: crate::Field<Self, crate::List<crate::Object<super::Worklog>>> =
        crate::Field::new("worklogs");
}
