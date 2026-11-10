#[derive(Clone, Copy, Debug)]
pub struct JiraLabelsField;

impl JiraLabelsField {
    pub const BULK_EDIT_MULTI_SELECT_FIELD_OPTION: crate::Field<Self, crate::Text> =
        crate::Field::new("bulkEditMultiSelectFieldOption");

    pub const FIELD_ID: crate::Field<Self, crate::Text> = crate::Field::new("fieldId");

    pub const LABELS: crate::Field<Self, crate::List<crate::Object<super::JiraLabelsInput>>> =
        crate::Field::new("labels");

    pub const LABEL_PROPERTIES: crate::Field<
        Self,
        crate::List<crate::Object<super::JiraLabelPropertiesInputJackson1>>,
    > = crate::Field::new("labelProperties");
}
