#[derive(Clone, Copy, Debug)]
pub struct Errors;

impl Errors {
    pub const ERRORS: crate::Field<Self, crate::Any> = crate::Field::new("errors");

    pub const ERROR_MESSAGES: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("errorMessages");
}
