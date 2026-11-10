#[derive(Clone, Copy, Debug)]
pub struct JiraDateTimeField;

impl JiraDateTimeField {
    pub const DATE_TIME: crate::Field<Self, crate::Object<super::JiraDateTimeInput>> =
        crate::Field::new("dateTime");

    pub const FIELD_ID: crate::Field<Self, crate::Text> = crate::Field::new("fieldId");
}
