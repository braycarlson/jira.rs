#[derive(Clone, Copy, Debug)]
pub struct JiraMultipleVersionPickerField;

impl JiraMultipleVersionPickerField {
    pub const BULK_EDIT_MULTI_SELECT_FIELD_OPTION: crate::Field<Self, crate::Text> =
        crate::Field::new("bulkEditMultiSelectFieldOption");

    pub const FIELD_ID: crate::Field<Self, crate::Text> = crate::Field::new("fieldId");

    pub const VERSIONS: crate::Field<Self, crate::List<crate::Object<super::JiraVersionField>>> =
        crate::Field::new("versions");
}
