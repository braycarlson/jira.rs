#[derive(Clone, Copy, Debug)]
pub struct FieldDetails;

impl FieldDetails {
    pub const CLAUSE_NAMES: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("clauseNames");

    pub const CUSTOM: crate::Field<Self, crate::Boolean> = crate::Field::new("custom");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const KEY: crate::Field<Self, crate::Text> = crate::Field::new("key");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
    pub const NAVIGABLE: crate::Field<Self, crate::Boolean> = crate::Field::new("navigable");
    pub const ORDERABLE: crate::Field<Self, crate::Boolean> = crate::Field::new("orderable");

    pub const SCHEMA: crate::Field<Self, crate::Object<super::JSONTypeBean>> =
        crate::Field::new("schema");

    pub const SCOPE: crate::Field<Self, crate::Object<super::Scope>> = crate::Field::new("scope");
    pub const SEARCHABLE: crate::Field<Self, crate::Boolean> = crate::Field::new("searchable");

    pub const UNTRANSLATED_NAME: crate::Field<Self, crate::Text> =
        crate::Field::new("untranslatedName");
}
