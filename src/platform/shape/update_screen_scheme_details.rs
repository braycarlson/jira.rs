#[derive(Clone, Copy, Debug)]
pub struct UpdateScreenSchemeDetails;

impl UpdateScreenSchemeDetails {
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");

    pub const SCREENS: crate::Field<Self, crate::Object<super::UpdateScreenTypes>> =
        crate::Field::new("screens");
}
