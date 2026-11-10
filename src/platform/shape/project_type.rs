#[derive(Clone, Copy, Debug)]
pub struct ProjectType;

impl ProjectType {
    pub const COLOR: crate::Field<Self, crate::Text> = crate::Field::new("color");

    pub const DESCRIPTION_I18N_KEY: crate::Field<Self, crate::Text> =
        crate::Field::new("descriptionI18nKey");

    pub const FORMATTED_KEY: crate::Field<Self, crate::Text> = crate::Field::new("formattedKey");
    pub const ICON: crate::Field<Self, crate::Text> = crate::Field::new("icon");
    pub const KEY: crate::Field<Self, crate::Text> = crate::Field::new("key");
}
