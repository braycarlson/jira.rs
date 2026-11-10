#[derive(Clone, Copy, Debug)]
pub struct ErrorResponse;

impl ErrorResponse {
    pub const ERROR_MESSAGE: crate::Field<Self, crate::Text> = crate::Field::new("errorMessage");

    pub const I18N_ERROR_MESSAGE: crate::Field<Self, crate::Object<super::I18nErrorMessage>> =
        crate::Field::new("i18nErrorMessage");
}
