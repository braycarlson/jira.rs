#[derive(Clone, Copy, Debug)]
pub struct SoftwareInfoDTO;

impl SoftwareInfoDTO {
    pub const BUILD_CHANGE_SET: crate::Field<Self, crate::Text> =
        crate::Field::new("buildChangeSet");

    pub const BUILD_DATE: crate::Field<Self, crate::Object<super::DateDTO>> =
        crate::Field::new("buildDate");

    pub const IS_LICENSED_FOR_USE: crate::Field<Self, crate::Boolean> =
        crate::Field::new("isLicensedForUse");

    pub const LINKS: crate::Field<Self, crate::Object<super::SelfLinkDTO>> =
        crate::Field::new("_links");

    pub const PLATFORM_VERSION: crate::Field<Self, crate::Text> =
        crate::Field::new("platformVersion");

    pub const VERSION: crate::Field<Self, crate::Text> = crate::Field::new("version");
}
