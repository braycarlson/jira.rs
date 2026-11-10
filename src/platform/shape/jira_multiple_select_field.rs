#[derive(Clone, Copy, Debug)]
pub struct JiraMultipleSelectField;

impl JiraMultipleSelectField {
    pub const FIELD_ID: crate::Field<Self, crate::Text> = crate::Field::new("fieldId");

    pub const OPTIONS: crate::Field<
        Self,
        crate::List<crate::Object<super::JiraSelectedOptionField>>,
    > = crate::Field::new("options");
}
