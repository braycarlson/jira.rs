#[derive(Clone, Copy, Debug)]
pub struct SystemAvatars;

impl SystemAvatars {
    pub const SYSTEM: crate::Field<Self, crate::List<crate::Object<super::Avatar>>> =
        crate::Field::new("system");
}
