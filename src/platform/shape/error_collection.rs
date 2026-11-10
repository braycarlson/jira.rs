#[derive(Clone, Copy, Debug)]
pub struct ErrorCollection;

impl ErrorCollection {
    pub const ERRORS: crate::Field<Self, crate::Map<crate::Text>> = crate::Field::new("errors");

    pub const ERROR_MESSAGES: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("errorMessages");

    pub const STATUS: crate::Field<Self, crate::Integer> = crate::Field::new("status");
}
