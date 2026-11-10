#[derive(Clone, Copy, Debug)]
pub struct ObjectListInclTypeAttributesEntryResult;

impl ObjectListInclTypeAttributesEntryResult {
    pub const HAS_MORE_RESULTS: crate::Field<Self, crate::Boolean> =
        crate::Field::new("hasMoreResults");

    pub const IS_LAST: crate::Field<Self, crate::Boolean> = crate::Field::new("isLast");
    pub const LAST: crate::Field<Self, crate::Boolean> = crate::Field::new("last");

    pub const OBJECT_TYPE_ATTRIBUTES: crate::Field<
        Self,
        crate::List<crate::Object<super::ObjectTypeAttribute>>,
    > = crate::Field::new("objectTypeAttributes");

    pub const RESULTS_MAX: crate::Field<Self, crate::Integer> = crate::Field::new("maxResults");
    pub const START_AT: crate::Field<Self, crate::Integer> = crate::Field::new("startAt");
    pub const TOTAL: crate::Field<Self, crate::Integer> = crate::Field::new("total");

    pub const VALUES: crate::Field<Self, crate::List<crate::Object<super::Object>>> =
        crate::Field::new("values");
}
