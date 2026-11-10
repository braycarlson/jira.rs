#[derive(Clone, Copy, Debug)]
pub struct AnnouncementBannerConfiguration;

impl AnnouncementBannerConfiguration {
    pub const HASH_ID: crate::Field<Self, crate::Text> = crate::Field::new("hashId");

    pub const IS_DISMISSIBLE: crate::Field<Self, crate::Boolean> =
        crate::Field::new("isDismissible");

    pub const IS_ENABLED: crate::Field<Self, crate::Boolean> = crate::Field::new("isEnabled");
    pub const MESSAGE: crate::Field<Self, crate::Text> = crate::Field::new("message");
    pub const VISIBILITY: crate::Field<Self, crate::Text> = crate::Field::new("visibility");
}
