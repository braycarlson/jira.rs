#[derive(Clone, Copy, Debug)]
pub struct CustomFieldPayload;

impl CustomFieldPayload {
    pub const CF_TYPE: crate::Field<Self, crate::Text> = crate::Field::new("cfType");
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
    pub const ON_CONFLICT: crate::Field<Self, crate::Text> = crate::Field::new("onConflict");

    pub const PCRI: crate::Field<Self, crate::Object<super::ProjectCreateResourceIdentifier>> =
        crate::Field::new("pcri");

    pub const SCOPE: crate::Field<Self, crate::Text> = crate::Field::new("scope");
    pub const SEARCHER_KEY: crate::Field<Self, crate::Text> = crate::Field::new("searcherKey");
}
