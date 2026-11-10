#[derive(Clone, Copy, Debug)]
pub struct DashboardGadgetSettings;

impl DashboardGadgetSettings {
    pub const COLOR: crate::Field<Self, crate::Text> = crate::Field::new("color");

    pub const IGNORE_URI_AND_MODULE_KEY_VALIDATION: crate::Field<Self, crate::Boolean> =
        crate::Field::new("ignoreUriAndModuleKeyValidation");

    pub const MODULE_KEY: crate::Field<Self, crate::Text> = crate::Field::new("moduleKey");

    pub const POSITION: crate::Field<Self, crate::Object<super::DashboardGadgetPosition>> =
        crate::Field::new("position");

    pub const TITLE: crate::Field<Self, crate::Text> = crate::Field::new("title");
    pub const URI: crate::Field<Self, crate::Text> = crate::Field::new("uri");
}
