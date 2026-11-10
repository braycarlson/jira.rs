#[derive(Clone, Copy, Debug)]
pub struct RequestValidationResultDTO;

impl RequestValidationResultDTO {
    pub const ERROR_MESSAGE: crate::Field<Self, crate::Text> = crate::Field::new("errorMessage");

    pub const ERROR_MESSAGES: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("errorMessages");

    pub const FIELD_ERRORS: crate::Field<
        Self,
        crate::List<crate::Object<super::RequestFieldValidationErrorDTO>>,
    > = crate::Field::new("fieldErrors");

    pub const FORM_ERRORS: crate::Field<
        Self,
        crate::List<crate::Object<super::FormValidationErrorDTO>>,
    > = crate::Field::new("formErrors");

    pub const REASON_KEY: crate::Field<Self, crate::Text> = crate::Field::new("reasonKey");
    pub const VALID: crate::Field<Self, crate::Boolean> = crate::Field::new("valid");
}
