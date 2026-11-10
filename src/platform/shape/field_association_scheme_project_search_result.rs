#[derive(Clone, Copy, Debug)]
pub struct FieldAssociationSchemeProjectSearchResult;

impl FieldAssociationSchemeProjectSearchResult {
    pub const AVATAR_URLS: crate::Field<Self, crate::Map<crate::Text>> =
        crate::Field::new("avatarUrls");

    pub const DELETED: crate::Field<Self, crate::Boolean> = crate::Field::new("deleted");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const KEY: crate::Field<Self, crate::Text> = crate::Field::new("key");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
}
