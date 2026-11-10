#[derive(Clone, Copy, Debug)]
pub struct Locale;

impl Locale {
    pub const LOCALE: crate::Field<Self, crate::Text> = crate::Field::new("locale");
}
