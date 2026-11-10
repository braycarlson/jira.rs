#[derive(Clone, Copy, Debug)]
pub struct ImportSourceResponseImportSourceOTEntries;

impl ImportSourceResponseImportSourceOTEntries {
    pub const CREATED: crate::Field<Self, crate::Text> = crate::Field::new("created");
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const EMPTY_VALUES: crate::Field<Self, crate::Text> = crate::Field::new("emptyValues");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");

    pub const IMPORT_SOURCE_ID: crate::Field<Self, crate::Text> =
        crate::Field::new("importSourceId");

    pub const IMPORT_SOURCE_OT_ATTRIBUTE_ENTRIES: crate::Field<
        Self,
        crate::List<
            crate::Object<
                super::ImportSourceResponseImportSourceOTEntriesImportSourceOTAttrEntries,
            >,
        >,
    > = crate::Field::new("importSourceOTAttrEntries");

    pub const IMPORT_STATUS: crate::Field<
        Self,
        crate::Object<super::ImportSourceResponseImportSourceOTEntriesImportStatus>,
    > = crate::Field::new("importStatus");

    pub const MATCH_IDENTIFIER_IGNORE_CASE: crate::Field<Self, crate::Boolean> =
        crate::Field::new("matchIdentifierIgnoreCase");

    pub const OBJECT_TYPE: crate::Field<
        Self,
        crate::Object<super::ImportSourceResponseImportSourceOTEntriesObjectType>,
    > = crate::Field::new("objectType");

    pub const PARENT_IMPORT_SOURCE_OT_ID: crate::Field<Self, crate::Text> =
        crate::Field::new("parentImportSourceOTId");

    pub const SELECTOR: crate::Field<Self, crate::Text> = crate::Field::new("selector");
    pub const SELECTOR_IQL: crate::Field<Self, crate::Text> = crate::Field::new("selectorIQL");

    pub const SELECTOR_QL_QUERY: crate::Field<Self, crate::Text> =
        crate::Field::new("selectorQlQuery");

    pub const UNKNOWN_VALUES: crate::Field<Self, crate::Text> = crate::Field::new("unknownValues");
    pub const UPDATED: crate::Field<Self, crate::Text> = crate::Field::new("updated");
}
