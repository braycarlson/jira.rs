#[derive(Clone, Copy, Debug)]
pub struct ScreenPayload;

impl ScreenPayload {
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");

    pub const PCRI: crate::Field<Self, crate::Object<super::ProjectCreateResourceIdentifier>> =
        crate::Field::new("pcri");

    pub const TABS: crate::Field<Self, crate::List<crate::Object<super::TabPayload>>> =
        crate::Field::new("tabs");
}
