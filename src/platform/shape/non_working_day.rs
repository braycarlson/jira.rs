#[derive(Clone, Copy, Debug)]
pub struct NonWorkingDay;

impl NonWorkingDay {
    pub const ID: crate::Field<Self, crate::Integer> = crate::Field::new("id");
    pub const ISO8601_DATE: crate::Field<Self, crate::Text> = crate::Field::new("iso8601Date");
}
