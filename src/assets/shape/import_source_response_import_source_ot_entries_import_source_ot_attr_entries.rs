#[derive(Clone, Copy, Debug)]
pub struct ImportSourceResponseImportSourceOTEntriesImportSourceOTAttrEntries;

impl ImportSourceResponseImportSourceOTEntriesImportSourceOTAttrEntries {
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");

    pub const OBJECT_TYPE_ATTRIBUTE_DISPLAY_NAME: crate::Field<Self, crate::Text> =
        crate::Field::new("objectTypeAttributeDisplayName");
}
