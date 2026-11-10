#[derive(Clone, Copy, Debug)]
pub struct JiraCascadingSelectField;

impl JiraCascadingSelectField {
    pub const CHILD_OPTION_VALUE: crate::Field<
        Self,
        crate::Object<super::JiraSelectedOptionField>,
    > = crate::Field::new("childOptionValue");

    pub const FIELD_ID: crate::Field<Self, crate::Text> = crate::Field::new("fieldId");

    pub const PARENT_OPTION_VALUE: crate::Field<
        Self,
        crate::Object<super::JiraSelectedOptionField>,
    > = crate::Field::new("parentOptionValue");
}
