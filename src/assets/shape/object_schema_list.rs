#[derive(Clone, Copy, Debug)]
pub struct ObjectSchemaList;

impl ObjectSchemaList {
    pub const IS_LAST: crate::Field<Self, crate::Boolean> = crate::Field::new("isLast");
    pub const LAST: crate::Field<Self, crate::Boolean> = crate::Field::new("last");
    pub const RESULTS_MAX: crate::Field<Self, crate::Integer> = crate::Field::new("maxResults");
    pub const START_AT: crate::Field<Self, crate::Integer> = crate::Field::new("startAt");
    pub const TOTAL: crate::Field<Self, crate::Integer> = crate::Field::new("total");

    pub const VALUES: crate::Field<Self, crate::List<crate::Object<super::ObjectSchema>>> =
        crate::Field::new("values");
}
