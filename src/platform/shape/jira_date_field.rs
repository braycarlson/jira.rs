#[derive(Clone, Copy, Debug)]
pub struct JiraDateField;

impl JiraDateField {
    pub const DATE: crate::Field<Self, crate::Object<super::JiraDateInput>> =
        crate::Field::new("date");

    pub const FIELD_ID: crate::Field<Self, crate::Text> = crate::Field::new("fieldId");
}
