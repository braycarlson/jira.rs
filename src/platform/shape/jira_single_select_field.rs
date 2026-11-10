#[derive(Clone, Copy, Debug)]
pub struct JiraSingleSelectField;

impl JiraSingleSelectField {
    pub const FIELD_ID: crate::Field<Self, crate::Text> = crate::Field::new("fieldId");

    pub const OPTION: crate::Field<Self, crate::Object<super::JiraSelectedOptionField>> =
        crate::Field::new("option");
}
