#[derive(Clone, Copy, Debug)]
pub struct GetAllQuickFiltersReply;

impl GetAllQuickFiltersReply {
    pub const IS_LAST: crate::Field<Self, crate::Boolean> = crate::Field::new("isLast");
    pub const RESULTS_MAX: crate::Field<Self, crate::Integer> = crate::Field::new("maxResults");
    pub const START_AT: crate::Field<Self, crate::Integer> = crate::Field::new("startAt");
    pub const TOTAL: crate::Field<Self, crate::Integer> = crate::Field::new("total");

    pub const VALUES: crate::Field<
        Self,
        crate::List<crate::Object<super::GetAllQuickFiltersReplyValues>>,
    > = crate::Field::new("values");
}
