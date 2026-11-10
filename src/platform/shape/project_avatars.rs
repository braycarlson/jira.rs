#[derive(Clone, Copy, Debug)]
pub struct ProjectAvatars;

impl ProjectAvatars {
    pub const CUSTOM: crate::Field<Self, crate::List<crate::Object<super::Avatar>>> =
        crate::Field::new("custom");

    pub const SYSTEM: crate::Field<Self, crate::List<crate::Object<super::Avatar>>> =
        crate::Field::new("system");
}
