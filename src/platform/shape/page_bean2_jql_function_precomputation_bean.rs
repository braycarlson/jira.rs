#[derive(Clone, Copy, Debug)]
pub struct PageBean2JQLFunctionPrecomputationBean;

impl PageBean2JQLFunctionPrecomputationBean {
    pub const IS_LAST: crate::Field<Self, crate::Boolean> = crate::Field::new("isLast");
    pub const PAGE_NEXT: crate::Field<Self, crate::Text> = crate::Field::new("nextPage");
    pub const RESULTS_MAX: crate::Field<Self, crate::Integer> = crate::Field::new("maxResults");
    pub const SELF: crate::Field<Self, crate::Text> = crate::Field::new("self");
    pub const START_AT: crate::Field<Self, crate::Integer> = crate::Field::new("startAt");
    pub const TOTAL: crate::Field<Self, crate::Integer> = crate::Field::new("total");

    pub const VALUES: crate::Field<
        Self,
        crate::List<crate::Object<super::JQLFunctionPrecomputationBean>>,
    > = crate::Field::new("values");
}
