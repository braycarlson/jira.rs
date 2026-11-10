#[derive(Clone, Copy, Debug)]
pub struct JiraMultiSelectComponentField;

impl JiraMultiSelectComponentField {
    pub const BULK_EDIT_MULTI_SELECT_FIELD_OPTION: crate::Field<Self, crate::Text> =
        crate::Field::new("bulkEditMultiSelectFieldOption");

    pub const COMPONENTS: crate::Field<
        Self,
        crate::List<crate::Object<super::JiraComponentField>>,
    > = crate::Field::new("components");

    pub const FIELD_ID: crate::Field<Self, crate::Text> = crate::Field::new("fieldId");
}
