#[derive(Clone, Copy, Debug)]
pub struct FormValidationErrorResponseDTO;

impl FormValidationErrorResponseDTO {
    pub const ERRORS: crate::Field<
        Self,
        crate::List<crate::Object<super::FormValidationErrorDTO>>,
    > = crate::Field::new("errors");

    pub const ERROR_MESSAGE: crate::Field<Self, crate::Text> = crate::Field::new("errorMessage");

    pub const I18N_ERROR_MESSAGE: crate::Field<Self, crate::Object<super::I18nErrorMessageDTO>> =
        crate::Field::new("i18nErrorMessage");
}
