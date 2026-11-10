#[derive(Clone, Copy, Debug)]
pub struct IssueTypeDetails;

impl IssueTypeDetails {
    pub const AVATAR_ID: crate::Field<Self, crate::Integer> = crate::Field::new("avatarId");
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const ENTITY_ID: crate::Field<Self, crate::Text> = crate::Field::new("entityId");

    pub const HIERARCHY_LEVEL: crate::Field<Self, crate::Integer> =
        crate::Field::new("hierarchyLevel");

    pub const ICON_URL: crate::Field<Self, crate::Text> = crate::Field::new("iconUrl");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
    pub const SCOPE: crate::Field<Self, crate::Object<super::Scope>> = crate::Field::new("scope");
    pub const SELF: crate::Field<Self, crate::Text> = crate::Field::new("self");
    pub const SUBTASK: crate::Field<Self, crate::Boolean> = crate::Field::new("subtask");

    pub const UNTRANSLATED_NAME: crate::Field<Self, crate::Text> =
        crate::Field::new("untranslatedName");
}
