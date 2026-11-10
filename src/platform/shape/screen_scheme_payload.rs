#[derive(Clone, Copy, Debug)]
pub struct ScreenSchemePayload;

impl ScreenSchemePayload {
    pub const DEFAULT_SCREEN: crate::Field<
        Self,
        crate::Object<super::ProjectCreateResourceIdentifier>,
    > = crate::Field::new("defaultScreen");

    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");

    pub const PCRI: crate::Field<Self, crate::Object<super::ProjectCreateResourceIdentifier>> =
        crate::Field::new("pcri");

    pub const SCREENS: crate::Field<
        Self,
        crate::Map<crate::Object<super::ProjectCreateResourceIdentifier>>,
    > = crate::Field::new("screens");
}
