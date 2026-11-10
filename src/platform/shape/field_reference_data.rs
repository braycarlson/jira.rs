#[derive(Clone, Copy, Debug)]
pub struct FieldReferenceData;

impl FieldReferenceData {
    pub const AUTO: crate::Field<Self, crate::Text> = crate::Field::new("auto");
    pub const CFID: crate::Field<Self, crate::Text> = crate::Field::new("cfid");
    pub const DEPRECATED: crate::Field<Self, crate::Text> = crate::Field::new("deprecated");

    pub const DEPRECATED_SEARCHER_KEY: crate::Field<Self, crate::Text> =
        crate::Field::new("deprecatedSearcherKey");

    pub const DISPLAY_NAME: crate::Field<Self, crate::Text> = crate::Field::new("displayName");

    pub const OPERATORS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("operators");

    pub const ORDERABLE: crate::Field<Self, crate::Text> = crate::Field::new("orderable");
    pub const SEARCHABLE: crate::Field<Self, crate::Text> = crate::Field::new("searchable");
    pub const TYPES: crate::Field<Self, crate::List<crate::Text>> = crate::Field::new("types");
    pub const VALUE: crate::Field<Self, crate::Text> = crate::Field::new("value");
}
