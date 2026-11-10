#[derive(Clone, Copy, Debug)]
pub struct SimpleErrorCollection;

impl SimpleErrorCollection {
    pub const ERRORS: crate::Field<Self, crate::Map<crate::Text>> = crate::Field::new("errors");

    pub const ERROR_MESSAGES: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("errorMessages");

    pub const HTTP_STATUS_CODE: crate::Field<Self, crate::Integer> =
        crate::Field::new("httpStatusCode");
}
