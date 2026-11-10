#[derive(Clone, Copy, Debug)]
pub struct DateRangeFilterRequest;

impl DateRangeFilterRequest {
    pub const DATE_AFTER: crate::Field<Self, crate::Text> = crate::Field::new("dateAfter");
    pub const DATE_BEFORE: crate::Field<Self, crate::Text> = crate::Field::new("dateBefore");
}
