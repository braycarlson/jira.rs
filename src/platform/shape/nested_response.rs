#[derive(Clone, Copy, Debug)]
pub struct NestedResponse;

impl NestedResponse {
    pub const ERROR_COLLECTION: crate::Field<Self, crate::Object<super::ErrorCollection>> =
        crate::Field::new("errorCollection");

    pub const STATUS: crate::Field<Self, crate::Integer> = crate::Field::new("status");

    pub const WARNING_COLLECTION: crate::Field<Self, crate::Object<super::WarningCollection>> =
        crate::Field::new("warningCollection");
}
