#[derive(Clone, Copy, Debug)]
pub struct I18nErrorMessage;

impl I18nErrorMessage {
    pub const I18N_KEY: crate::Field<Self, crate::Text> = crate::Field::new("i18nKey");

    pub const PARAMETERS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("parameters");
}
