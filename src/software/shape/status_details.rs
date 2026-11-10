#[derive(Clone, Copy, Debug)]
pub struct StatusDetails;

impl StatusDetails {
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const ICON_URL: crate::Field<Self, crate::Text> = crate::Field::new("iconUrl");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");

    pub const SCOPE: crate::Field<Self, crate::Object<super::StatusDetailsScope>> =
        crate::Field::new("scope");

    pub const SELF: crate::Field<Self, crate::Text> = crate::Field::new("self");

    pub const STATUS_CATEGORY: crate::Field<
        Self,
        crate::Object<super::StatusDetailsStatusCategory>,
    > = crate::Field::new("statusCategory");

    pub const UNTRANSLATED_NAME: crate::Field<Self, crate::Text> =
        crate::Field::new("untranslatedName");
}
