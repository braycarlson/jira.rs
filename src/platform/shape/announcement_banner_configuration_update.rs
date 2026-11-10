#[derive(Clone, Copy, Debug)]
pub struct AnnouncementBannerConfigurationUpdate;

impl AnnouncementBannerConfigurationUpdate {
    pub const IS_DISMISSIBLE: crate::Field<Self, crate::Boolean> =
        crate::Field::new("isDismissible");

    pub const IS_ENABLED: crate::Field<Self, crate::Boolean> = crate::Field::new("isEnabled");
    pub const MESSAGE: crate::Field<Self, crate::Text> = crate::Field::new("message");
    pub const VISIBILITY: crate::Field<Self, crate::Text> = crate::Field::new("visibility");
}
