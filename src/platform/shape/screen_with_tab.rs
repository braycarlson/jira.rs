#[derive(Clone, Copy, Debug)]
pub struct ScreenWithTab;

impl ScreenWithTab {
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const ID: crate::Field<Self, crate::Integer> = crate::Field::new("id");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
    pub const SCOPE: crate::Field<Self, crate::Object<super::Scope>> = crate::Field::new("scope");

    pub const TAB: crate::Field<Self, crate::Object<super::ScreenableTab>> =
        crate::Field::new("tab");
}
