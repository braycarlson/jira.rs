#[derive(Clone, Copy, Debug)]
pub struct PageOfDashboards;

impl PageOfDashboards {
    pub const DASHBOARDS: crate::Field<Self, crate::List<crate::Object<super::Dashboard>>> =
        crate::Field::new("dashboards");

    pub const NEXT: crate::Field<Self, crate::Text> = crate::Field::new("next");
    pub const PREVIOUS: crate::Field<Self, crate::Text> = crate::Field::new("prev");
    pub const RESULTS_MAX: crate::Field<Self, crate::Integer> = crate::Field::new("maxResults");
    pub const START_AT: crate::Field<Self, crate::Integer> = crate::Field::new("startAt");
    pub const TOTAL: crate::Field<Self, crate::Integer> = crate::Field::new("total");
}
